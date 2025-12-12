pub struct Solution;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Mention {
    All,
    Here,
    Users(Vec<usize>),
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    Offline { timestamp: i32, id: usize },
    Message { timestamp: i32, mention: Mention },
}

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut events: Vec<Event> = events.iter().map(Event::new).collect();
        events.sort();

        let mut last_offlines: Vec<Option<i32>> = vec![None; number_of_users as usize];
        let mut mentions_count: Vec<i32> = vec![0; number_of_users as usize];

        for event in events {
            match event {
                Event::Offline { timestamp, id } => {
                    last_offlines[id] = Some(timestamp);
                }
                Event::Message { timestamp, mention } => match mention {
                    Mention::All => {
                        mentions_count.iter_mut().for_each(|c| *c += 1);
                    }
                    Mention::Here => {
                        for (index, count) in mentions_count.iter_mut().enumerate() {
                            if let Some(last_offline) = last_offlines[index]
                                && timestamp - last_offline < 60
                            {
                            } else {
                                *count += 1;
                            }
                        }
                    }
                    Mention::Users(ids) => {
                        ids.iter().for_each(|id| mentions_count[*id] += 1);
                    }
                },
            }
        }

        mentions_count
    }
}

impl Event {
    pub fn new(raw: &Vec<String>) -> Self {
        let event_type = &raw[0];
        let timestamp = raw[1].parse::<i32>().unwrap();
        let content = &raw[2];

        match event_type.as_str() {
            "MESSAGE" => {
                let mention = match content.as_str() {
                    "ALL" => Mention::All,
                    "HERE" => Mention::Here,
                    _ => {
                        let ids: Vec<usize> = content
                            .split(' ')
                            .map(|s| s.strip_prefix("id").unwrap().parse::<usize>().unwrap())
                            .collect();
                        Mention::Users(ids)
                    }
                };
                Event::Message { timestamp, mention }
            }
            _ => {
                let id = content.parse::<usize>().unwrap();
                Event::Offline { timestamp, id }
            }
        }
    }

    pub fn get_timestamp(&self) -> i32 {
        match self {
            Event::Message { timestamp, .. } => *timestamp,
            Event::Offline { timestamp, .. } => *timestamp,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_timestamp()
            .cmp(&other.get_timestamp())
            .then(match (self, other) {
                (Event::Offline { .. }, Event::Message { .. }) => std::cmp::Ordering::Less,
                (Event::Message { .. }, Event::Offline { .. }) => std::cmp::Ordering::Greater,
                (Event::Offline { id: id_a, .. }, Event::Offline { id: id_b, .. }) => {
                    id_a.cmp(id_b)
                }
                (
                    Event::Message {
                        mention: mention_a, ..
                    },
                    Event::Message {
                        mention: mention_b, ..
                    },
                ) => mention_a.cmp(mention_b),
            })
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
