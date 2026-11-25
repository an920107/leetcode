#!/usr/bin/env python3
"""把 LeetCode 的 testcase.txt（兩行 JSON）轉成 Rust 回歸測試。

用法:
    python3 generate_test.py [輸入 testcase.txt] [輸出 tests/testcase.rs]

預設讀取同目錄的 testcase.txt，輸出到同目錄的 tests/testcase.rs。

期望答案是使用 Python 的任意精度整數計算的，所以可以用來抓出
「前綴乘積在 i32 溢位後歸零 / 答案算錯」這類 bug。
"""

import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent

# 產出的測試會 use 這個 crate
CRATE_NAME = "product_of_the_last_k_numbers"

PER_LINE_OPS = 8
PER_LINE_ARGS = 16
PER_LINE_EXPECTED = 16

I32_MAX = 2**31 - 1


def load_operations(path: Path) -> tuple[list[str], list[list[int]]]:
    with path.open() as f:
        operations = json.loads(f.readline())
        arguments = json.loads(f.readline())

    if len(operations) != len(arguments):
        raise SystemExit(
            f"{path}: 兩行長度不一致 ({len(operations)} vs {len(arguments)})"
        )
    return operations, arguments


def compute_expected(
    operations: list[str], arguments: list[list[int]]
) -> list[int]:
    """回傳每個 getProduct 的精確答案，同時驗證輸入是否符合題目限制。"""
    nums: list[int] = []
    expected: list[int] = []

    for i, (op, args) in enumerate(zip(operations, arguments)):
        if op == "ProductOfNumbers":
            if args or i != 0:
                raise SystemExit(f"op {i}: ProductOfNumbers 只該出現在開頭")
            nums = []
        elif op == "add":
            if len(args) != 1 or not 0 <= args[0] <= 100:
                raise SystemExit(f"op {i}: 不合法的 add 參數 {args}")
            nums.append(args[0])
        elif op == "getProduct":
            if len(args) != 1:
                raise SystemExit(f"op {i}: 不合法的 getProduct 參數 {args}")
            k = args[0]
            if not 1 <= k <= len(nums):
                raise SystemExit(f"op {i}: k={k} 超過目前元素數 {len(nums)}")

            window = nums[-k:]
            if 0 in window:
                product = 0
            else:
                product = 1
                for n in window:
                    product *= n

            if not 0 <= product <= I32_MAX:
                raise SystemExit(
                    f"op {i}: 答案 {product} 超出題目保證的 32-bit 範圍"
                )
            expected.append(product)
        else:
            raise SystemExit(f"op {i}: 未知操作 {op!r}")

    return expected


def format_items(items, per_line: int, render) -> str:
    lines = []
    for i in range(0, len(items), per_line):
        chunk = ", ".join(render(x) for x in items[i : i + per_line])
        lines.append(f"    {chunk},")
    return "\n".join(lines)


def render(operations, arguments, expected) -> str:
    ops_src = format_items(operations, PER_LINE_OPS, lambda s: f'"{s}"')
    args_src = format_items(
        arguments,
        PER_LINE_ARGS,
        lambda a: "&[]" if not a else "&[" + ", ".join(map(str, a)) + "]",
    )
    expected_src = format_items(expected, PER_LINE_EXPECTED, str)

    return f"""//! 由 generate_test.py 依 testcase.txt 自動產生，請勿手改。
//!
//! 重新產生：`python3 generate_test.py`

use {CRATE_NAME}::ProductOfNumbers;

const OPERATIONS: &[&str] = &[
{ops_src}
];

const ARGUMENTS: &[&[i32]] = &[
{args_src}
];

/// 每個 `getProduct` 的期望答案（以精確整數計算）。
const EXPECTED: &[i32] = &[
{expected_src}
];

#[test]
fn leetcode_testcase() {{
    assert_eq!(OPERATIONS.len(), ARGUMENTS.len());

    let mut obj = ProductOfNumbers::new();
    let mut expected = EXPECTED.iter();

    for (i, (op, args)) in OPERATIONS.iter().zip(ARGUMENTS).enumerate() {{
        match *op {{
            "ProductOfNumbers" => obj = ProductOfNumbers::new(),
            "add" => obj.add(args[0]),
            "getProduct" => {{
                let got = obj.get_product(args[0]);
                let want = *expected
                    .next()
                    .unwrap_or_else(|| panic!("op {{i}}: EXPECTED 數量不足"));
                assert_eq!(got, want, "op {{i}}: get_product({{}})", args[0]);
            }}
            other => panic!("op {{i}}: 未知操作 {{other}}"),
        }}
    }}

    assert!(expected.next().is_none(), "還有未使用的 EXPECTED");
}}
"""


def main() -> None:
    input_path = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / "testcase.txt"
    output_path = (
        Path(sys.argv[2]) if len(sys.argv) > 2 else HERE / "tests" / "testcase.rs"
    )

    operations, arguments = load_operations(input_path)
    expected = compute_expected(operations, arguments)

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(render(operations, arguments, expected))

    print(
        f"ops={len(operations)} getProduct={len(expected)} -> {output_path} "
        f"({output_path.stat().st_size} bytes)"
    )


if __name__ == "__main__":
    main()
