cat file.txt | awk '
{
    max_cols = 0
    for (i = 1; i <= NF; i++) {
        data[i, NR] = $i
    }
    if (NF > max_cols) {
        max_cols = NF
    }
}

END {
    for (i = 1; i <= max_cols; i++) {
        for (j = 1; j <= NR; j++) {
            printf "%s", data[i, j]
            if (j < NR) {
                printf " "
            } 
        }
        printf "\n"
    }
}
'
