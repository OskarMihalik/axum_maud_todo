// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod todo
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpdateTodoParams < T1 : cornucopia_async::StringSql,> { pub name : T1,pub id : i32,}#[derive(Clone,Copy, Debug)] pub struct SelectTodosParams < > { pub limit : i64,pub offset : i64,}#[derive( Debug, Clone, PartialEq, )] pub struct SelectTodos
{ pub id : i32,pub name : String,}pub struct SelectTodosBorrowed < 'a >
{ pub id : i32,pub name : &'a str,} impl < 'a > From < SelectTodosBorrowed <
'a >> for SelectTodos
{
    fn
    from(SelectTodosBorrowed { id,name,} : SelectTodosBorrowed < 'a >)
    -> Self { Self { id,name: name.into(),} }
}pub struct SelectTodosQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> SelectTodosBorrowed,
    mapper : fn(SelectTodosBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > SelectTodosQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(SelectTodosBorrowed) -> R) -> SelectTodosQuery
    < 'a, C, R, N >
    {
        SelectTodosQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn update_todo() -> UpdateTodoStmt
{ UpdateTodoStmt(cornucopia_async :: private :: Stmt :: new("UPDATE todo SET name=($1) WHERE id=($2)")) } pub
struct UpdateTodoStmt(cornucopia_async :: private :: Stmt) ; impl
UpdateTodoStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [name,id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, UpdateTodoParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for UpdateTodoStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UpdateTodoParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.name,& params.id,) ) }
}pub fn delete_todo() -> DeleteTodoStmt
{ DeleteTodoStmt(cornucopia_async :: private :: Stmt :: new("DELETE FROM todo WHERE id=($1)")) } pub
struct DeleteTodoStmt(cornucopia_async :: private :: Stmt) ; impl
DeleteTodoStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [id,]) .await
} }pub fn insert_todo() -> InsertTodoStmt
{ InsertTodoStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO todo (name) values ($1)")) } pub
struct InsertTodoStmt(cornucopia_async :: private :: Stmt) ; impl
InsertTodoStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [name,]) .await
} }pub fn select_todos() -> SelectTodosStmt
{ SelectTodosStmt(cornucopia_async :: private :: Stmt :: new("SELECT id, name FROM todo 
ORDER BY todo.id
LIMIT $1
OFFSET $2")) } pub
struct SelectTodosStmt(cornucopia_async :: private :: Stmt) ; impl
SelectTodosStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
limit : & 'a i64,offset : & 'a i64,) -> SelectTodosQuery < 'a, C,
SelectTodos, 2 >
{
    SelectTodosQuery
    {
        client, params : [limit,offset,], stmt : & mut self.0, extractor :
        | row | { SelectTodosBorrowed { id : row.get(0),name : row.get(1),} }, mapper : | it | { <SelectTodos>::from(it) },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, SelectTodosParams < >, SelectTodosQuery < 'a,
C, SelectTodos, 2 >, C > for SelectTodosStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    SelectTodosParams < >) -> SelectTodosQuery < 'a, C,
    SelectTodos, 2 >
    { self.bind(client, & params.limit,& params.offset,) }
}}}