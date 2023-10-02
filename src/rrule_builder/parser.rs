use chrono::NaiveDateTime;

pub fn process_vec_field<T, U, F>(
    data: &[T],
    transform: F,
    field_name: &str,
) -> Result<Vec<U>, String>
where
    F: Fn(&T) -> Result<U, String>,
{
    data.iter()
        .map(|item| transform(item).map_err(|e| format!("[{}] {}", field_name, e)))
        .collect()
}

pub fn process_datetime_field(
    field_value: &str,
    field_name: &str,
) -> Result<NaiveDateTime, String> {
    field_value
        .parse::<NaiveDateTime>()
        .map_err(|_| format!("[{}] Invalid datetime format", field_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_field_success() {
        let data = vec![1, 2, 3, 4, 5];
        let result: Result<Vec<i8>, String> =
            process_vec_field(&data, |x| Ok((*x) as i8), "field_name");
        assert_eq!(result, Ok(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_process_field_failure() {
        let data = vec![1, 2, 3, 4, 5];
        let result: Result<Vec<i8>, String> = process_vec_field(
            &data,
            |x| {
                if *x == 3 {
                    Err("Error on 3".to_string())
                } else {
                    Ok((*x) as i8)
                }
            },
            "field_name",
        );
        assert_eq!(result, Err("[field_name] Error on 3".to_string()));
    }
    #[test]
    fn test_process_datetime_field_valid() {
        let test_date = "2023-09-14T12:00:00";
        let result = process_datetime_field(test_date, "dt_start");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDateTime::parse_from_str(test_date, "%Y-%m-%dT%H:%M:%S").unwrap()
        );
    }

    #[test]
    fn test_process_datetime_field_invalid_format() {
        let test_date = "2023-09-14 12:00:00"; // Wrong format
        let result = process_datetime_field(test_date, "dt_start");
        assert!(result.is_err());
        assert_eq!(
            result,
            Err("[dt_start] Invalid datetime format".to_string())
        );
    }

    #[test]
    fn test_process_datetime_field_empty() {
        let test_date = "";
        let result = process_datetime_field(test_date, "dt_start");
        assert!(result.is_err());
        assert_eq!(
            result,
            Err("[dt_start] Invalid datetime format".to_string())
        );
    }
}
