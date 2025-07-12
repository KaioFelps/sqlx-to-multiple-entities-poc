#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "cargo_e", rename_all = "snake_case")]
pub enum Cargo {
    Aluno,
    Professor,
    Administrador,
}
