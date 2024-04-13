use design_patterns::strategy_di;

fn main() {
    let mysql_db = strategy_di::MySQLDatabase;
    let addition = strategy_di::AdditionStrategy;
    let data_service = strategy_di::DataService::new(mysql_db);
    let context = strategy_di::Context::new(addition, data_service);

    println!("{}", context.execute(2, 3));

    let postgres_db = strategy_di::PostgresDatabase;
    let subtraction = strategy_di::SubtractionStrategy;
    let data_service = strategy_di::DataService::new(postgres_db);
    let context = strategy_di::Context::new(subtraction, data_service);

    println!("{}", context.execute(10, 3));
}
