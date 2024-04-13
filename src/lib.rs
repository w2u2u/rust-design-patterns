pub mod strategy_di {
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
}
