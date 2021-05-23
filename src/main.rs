use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::*;

fn main() {
    let dialect = GenericDialect {}; // or AnsiDialect

    let sql = "SELECT * FROM x INNER JOIN y ON x.id = y.x_id INNER JOIN z ON y.z_id = z.id";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    let query = match &ast[0] {
        Statement::Query(q) => q,
        _ => panic!("Expected query")
    };

    let select = match &query.body {
        SetExpr::Select(s) => s,
        _ => panic!("Expected select")
    };

    let table = match &select.from[0].relation {
        TableFactor::Table { name, .. } => name,
        _ => panic!("Expected table")
    };

    println!("{:?}", table.0[0].value);
}
