pub mod u64_as_string_vec {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<Vec<u64>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(vec) => {
                let string_vec: Vec<String> = vec.iter().map(|&id| id.to_string()).collect();
                serializer.serialize_some(&string_vec)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u64>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<Vec<String>>::deserialize(deserializer)?;
        Ok(opt.map(|string_vec| {
            string_vec
                .into_iter()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        }))
    }
}
