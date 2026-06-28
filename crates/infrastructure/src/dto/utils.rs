use serde::{Deserialize, Deserializer};

/// 通用版本：将空字符串转为 None (仅针对 String)
pub fn empty_string_as_none<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }))
}

// 扩展思路：如果是数字类型，通常需要另一个函数，因为数字不能直接 trim
// pub fn empty_str_as_none_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> ...