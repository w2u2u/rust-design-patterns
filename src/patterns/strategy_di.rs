pub trait Database {
    fn query(&self, query: &str) -> String;
}

pub struct MySQLDatabase;

impl Database for MySQLDatabase {
    fn query(&self, query: &str) -> String {
        format!("MySQL: {}", query)
    }
}

pub struct PostgresDatabase;

impl Database for PostgresDatabase {
    fn query(&self, query: &str) -> String {
        format!("Postgres: {}", query)
    }
}

pub trait Strategy {
    fn execute_strategy(&self, a: i32, b: i32) -> i32;
}

pub struct AdditionStrategy;

impl Strategy for AdditionStrategy {
    fn execute_strategy(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub struct SubtractionStrategy;

impl Strategy for SubtractionStrategy {
    fn execute_strategy(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

pub struct DataService<D: Database> {
    db: D,
}

impl<D: Database> DataService<D> {
    pub fn new(db: D) -> Self {
        DataService { db }
    }

    fn get_data(&self, query: &str) -> String {
        self.db.query(query)
    }
}

pub struct Context<S: Strategy, D: Database> {
    strategy: S,
    data_service: DataService<D>,
}

impl<S: Strategy, D: Database> Context<S, D> {
    pub fn new(strategy: S, data_service: DataService<D>) -> Self {
        Context {
            strategy,
            data_service,
        }
    }

    pub fn execute(&self, a: i32, b: i32) -> String {
        let result = self.strategy.execute_strategy(a, b);

        self.data_service.get_data(&format!("SELECT {};", result))
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::strategy_di;

    #[test]
    fn test_strategy_di() {
        let mysql_db = strategy_di::MySQLDatabase;
        let addition = strategy_di::AdditionStrategy;
        let data_service = strategy_di::DataService::new(mysql_db);
        let context = strategy_di::Context::new(addition, data_service);

        assert_eq!(context.execute(2, 3), "MySQL: SELECT 5;");

        let postgres_db = strategy_di::PostgresDatabase;
        let subtraction = strategy_di::SubtractionStrategy;
        let data_service = strategy_di::DataService::new(postgres_db);
        let context = strategy_di::Context::new(subtraction, data_service);

        assert_eq!(context.execute(10, 3), "Postgres: SELECT 7;");
    }
}
