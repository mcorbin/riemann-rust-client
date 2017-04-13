use std::num;

#[derive(Debug)]
pub enum ParseError {
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError)
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError {
        ParseError::ParseInt(err)
    }
}

impl From<num::ParseFloatError> for ParseError {
    fn from(err: num::ParseFloatError) -> ParseError {
        ParseError::ParseFloat(err)
    }
}

pub fn parse_int(value: Option<&str>) -> Result<Option<i32>, num::ParseIntError> {
    match value {
        None => Ok(None),
        Some(v) => {
            let result = try!(v.parse::<i32>());
            Ok(Some(result))
        }
    }
}

pub fn parse_int64(value: Option<&str>) -> Result<Option<i64>, num::ParseIntError> {
    match value {
        None => Ok(None),
        Some(v) => {
            let result = try!(v.parse::<i64>());
            Ok(Some(result))
        }
    }
}

pub fn parse_float64(value: Option<&str>) -> Result<Option<f64>, num::ParseFloatError> {
    match value {
        None => Ok(None),
        Some(v) => {
            let result = try!(v.parse::<f64>());
            Ok(Some(result))
        }
    }
}

pub fn parse_float(value: Option<&str>) -> Result<Option<f32>, num::ParseFloatError> {
    match value {
        None => Ok(None),
        Some(v) => {
            let result = try!(v.parse::<f32>());
            Ok(Some(result))
        }
    }
}
