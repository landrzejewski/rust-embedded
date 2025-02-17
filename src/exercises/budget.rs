use std::env;
use std::fmt::{write, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DEPOSIT: &str = "Deposit";
const WITHDRAW: &str = "Withdraw";
const SEPARATOR: &str = ":";
const FILE_NAME: &str = "rusty_budget.txt";

enum OperationType {
    Deposit,
    Withdraw,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Deposit => write!(f, "{}", DEPOSIT),
            OperationType::Withdraw => write!(f, "{}", WITHDRAW),
        }
    }
}

impl TryFrom<&str> for OperationType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            DEPOSIT => Ok(OperationType::Deposit),
            WITHDRAW => Ok(OperationType::Withdraw),
            _ => Err(format!("Unknown operation type: {}", value)),
        }
    }
}

struct Operation {
    amount: f64,
    description: String,
    operation_type: OperationType,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.amount, self.description, self.operation_type
        )
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static PARTS_COUNT: usize = 3;
        let parts: Vec<&str> = value.split(SEPARATOR).collect();
        if parts.len() != PARTS_COUNT {
            return Err("Invalid input".to_string());
        }
        let amount: f64 = parts[0].parse().map_err(|e| "Invalid amount")?;
        let description = parts[1].to_string();
        let operation_type: OperationType = OperationType::try_from(parts[2])?;
        let operation = Operation {
            amount,
            description,
            operation_type,
        };
        Ok(operation)
    }
}

fn load() -> Vec<Operation> {
    let file = File::open(FILE_NAME).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut operations= Vec::new();
    for line in reader.lines() {
        let text = line.expect("Could not read line");
        let operation: Operation = text.into().expect("Invalid input");
        operations.push(operation);
    }
    operations
}

fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn run() {
    let ot = OperationType::try_from("Deposit");

    let o: Result<OperationType, String> = "Deposit".try_into();
}
