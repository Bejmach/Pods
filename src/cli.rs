use clap::{Parser};

#[derive(Parser)]
pub enum Command{
    Enable,
    Add(Notification),
    Get(GetFlags),
    Listen(GetFlags),
    Remove {#[arg(long)] id: u32},
    Clear,
}

#[derive(Parser)]
pub struct GetFlags{
    #[arg(long)]
    pub since: Option<u64>,

    #[arg(long)]
    pub all: bool,

    #[arg(long, short)]
    pub group_vec: bool,

    #[arg(long)]
    pub group_hash: bool,
}

#[derive(Parser)]
pub struct Notification{
    pub id: u32,
    pub app_name: String,
    #[arg(long)]
    pub app_icon: Option<String>,
    pub summary: String,
    pub body: String,
}
