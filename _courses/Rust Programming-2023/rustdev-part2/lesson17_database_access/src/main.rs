// This application connects to a MySQL database named MYSCHEMA on localhost.
// We've provided a Dockerfile that builds a Docker image housing MySQL.
// 
// To run the demo, open a Terminal window and run the following commands...
//
// 1. Build the MySQL Docker image, then run a MySQL Docker container:
//       docker build -t mysql-image .
//       docker run --name mysql-container -d -p 3306:3306 mysql-image
//
// 2. Run the Rust application:
//       cargo run
// 
// 3. Stop the Docker container and tidy up all Docker resources:
//       docker container rm -f mysql-container
//       docker image rm -f mysql-image

mod types;

use mysql::*;
use mysql::prelude::*;
use types::Employee;

fn main() {

    let mut conn = get_conn();

    create_temp_table(&mut conn);

    insert_employees(&mut conn);

    let employees = select_employees(&mut conn);
    
    println!("\nEmployees:");
    for e in employees {
        println!("{}", e);
    }
}

fn get_conn() -> PooledConn {

    let builder = OptsBuilder::new()
                    .ip_or_hostname(Some("localhost"))
                    .tcp_port(3306)
                    .db_name(Some("MYSCHEMA"))
                    .user(Some("root"))
                    .pass(Some("password"));

    let pool = mysql::Pool::new(builder).unwrap();

    pool.get_conn().unwrap()
}

fn create_temp_table(conn: &mut PooledConn) {

    conn.query_drop(
        r"CREATE TEMPORARY TABLE employees_temp (
            employee_id INT NOT NULL,
            name VARCHAR(50) NOT NULL,
            salary DOUBLE NOT NULL,
            region VARCHAR(50) NOT NULL,
            PRIMARY KEY (employee_id)
        )"
    ).unwrap()
}

fn insert_employees(conn: &mut PooledConn) {

    let employees = vec![
        Employee::new(1, String::from("Andy"),  25_000.0, String::from("Wales")),
        Employee::new(2, String::from("Jayne"), 35_000.0, String::from("Wales")),
        Employee::new(3, String::from("Emily"), 45_000.0, String::from("Scotland")),
        Employee::new(4, String::from("Tom"),   55_000.0, String::from("London"))
    ];

    conn.exec_batch(
        "INSERT INTO employees_temp (employee_id, name, salary, region) VALUES (:i, :n, :s, :r)",
        employees.iter().map(|e| params!{"i" => e.employee_id, "n" => &e.name, "s" => e.salary, "r" => &e.region})
    ).unwrap()
}

fn select_employees(conn: &mut PooledConn) -> Vec::<Employee> {

    conn.query_map(
        "SELECT employee_id, name, salary, region FROM employees_temp",
        |(i, n, s, r)| Employee::new(i, n, s, r) 
    ).unwrap()
}