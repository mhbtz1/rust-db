
#[derive(Debug)]
pub struct Column<'a, T> {
    components: &'a Vec<T>,
    metadata: String,
}

#[derive(Debug)]
pub struct Table<'a, 'b, S> {
    columns: &'a Vec<&'a Column<'b, S>>,
    metadata: String
}