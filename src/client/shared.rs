pub mod my_date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //println!("Serialize");
        if let Some(date) = date{
            let s = format!("{}", date.format(FORMAT));
            return serializer.serialize_str(&s);
        }
        serializer.serialize_str("")
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(s) = String::deserialize(deserializer) {
            match DateTime::parse_from_rfc3339(&s){
            Ok(time) => Ok(Some(time.with_timezone(&Utc))),
            Err(_) => Ok(None),
            }  
        }else{
            Ok(None)
        }
          
    }
}