

use serde::{Deserialize, Deserializer};

/// 自定义反序列化器：将空字符串或纯空格字符串转换为 None
pub fn empty_string_as_none<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // 先尝试反序列化为 Option<String>
    let opt = Option::<String>::deserialize(deserializer)?;

    // 如果存在，则检查 trim 后是否为空；如果不存在，直接返回 None
    Ok(opt
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    )
}