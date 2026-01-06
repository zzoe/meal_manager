//! 错误类型定义模块
//! 
//! 统一的应用错误处理，使用 `thiserror` 定义结构化错误类型

use thiserror::Error;

/// 应用程序错误枚举
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库相关错误
    #[error("数据库错误: {0}")]
    Database(#[from] DatabaseError),
    
    /// 分析逻辑错误
    #[error("分析错误: {0}")]
    Analysis(#[from] AnalysisError),
    
    /// 输入数据错误
    #[error("输入数据错误: {0}")]
    Input(String),
    
    /// 序列化/反序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// 正则表达式错误
    #[error("正则表达式错误: {0}")]
    Regex(#[from] regex::Error),
    
    /// 通道通信错误
    #[error("通道通信错误: {0}")]
    Channel(String),
    
    /// 其他未知错误
    #[error("未知错误: {0}")]
    Other(String),
}

/// 数据库错误类型
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("数据库连接失败: {0}")]
    Connection(String),
    
    #[error("表操作失败: {0}")]
    TableOperation(String),
    
    #[error("事务失败: {0}")]
    Transaction(String),
    
    #[error("数据不存在: {0}")]
    NotFound(String),
}

/// 分析逻辑错误类型
#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("无效的输入格式: {0}")]
    InvalidFormat(String),
    
    #[error("员工数据冲突: {0}")]
    EmployeeConflict(String),
    
    #[error("解析失败: {0}")]
    ParseError(String),
}

// 类型别名，简化使用
pub type Result<T> = std::result::Result<T, AppError>;

// 为外部错误类型实现转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Other(format!("IO错误: {}", err))
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Other(format!("内部错误: {}", err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Other(format!("操作失败: {}", err))
    }
}