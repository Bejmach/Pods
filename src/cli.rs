use clap::{Parser};

#[derive(Parser)]
pub enum Command{
    Enable,
    Add(Notification),
    Get(GetFlags),
    Listen(GetFlags),
    Clear,
}

#[derive(Parser)]
pub struct GetFlags{
    #[arg(long)]
    pub since: Option<u64>,

}

#[derive(Parser)]
pub struct Notification{
    pub app_name: String,
    #[arg(long)]
    pub app_icon: Option<String>,
    pub summary: String,
    pub body: String,
}
