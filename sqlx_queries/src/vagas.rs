use core::panic;

use poc_core::entidades::vaga::Vaga;
use sqlx::PgConnection;
use uuid::Uuid;

const BASE_SELECT_QUERY: &str = r#"SELECT
        v.*,
        p.id as p_id,
        p.title as p_title,
        p.content as p_content,
        p.created_at as p_created_at,
        coorde.id as coorde_id,
        coorde.nome as coorde_nome,
        coorde.email as coorde_email,
        coorde.senha_hash as coorde_senha_hash,
        coorde.curriculo_lattes as coorde_curriculo_lattes,
        coorde.cargo as coorde_cargo,
        coorde.ultimo_login_em as coorde_ultimo_login_em,
        coorde.atualizado_em as coorde_atualizado_em,
        coorde.criado_em as coorde_criado_em,
        vice.id as vice_id,
        vice.nome as vice_nome,
        vice.email as vice_email,
        vice.senha_hash as vice_senha_hash,
        vice.curriculo_lattes as vice_curriculo_lattes,
        vice.cargo as vice_cargo,
        vice.ultimo_login_em as vice_ultimo_login_em,
        vice.atualizado_em as vice_atualizado_em,
        vice.criado_em as vice_criado_em
    FROM vaga v
    INNER JOIN projeto p ON p.id = v.id_projeto
    INNER JOIN projeto_coordenadores rel_coord ON rel_coord.id_projeto = p.id AND rel_coord.tipo_coord = 'coord'
    INNER JOIN usuario coorde ON coorde.id = rel_coord.id_coord
    LEFT JOIN projeto_coordenadores rel_vice ON rel_vice.id_projeto = p.id AND rel_vice.tipo_coord = 'vice'
    LEFT JOIN usuario vice ON vice.id = rel_vice.id_coord"#;

pub struct VagasRepo<'this> {
    pub db: &'this mut PgConnection,
}

impl VagasRepo<'_> {
    pub async fn salvar_vaga(&mut self, vaga: &Vaga) {
        sqlx::query!(
            r#"INSERT INTO vaga (
                id,
                id_projeto,
                titulo_customizado,
                detalhes,
                quantidade,
                horas_p_semana
            ) VALUES ($1, $2, $3, $4, $5, $6)"#,
            &vaga.id,
            &vaga.projeto.projeto.id,
            vaga.titulo_customizado.as_ref(),
            &vaga.detalhes,
            vaga.quantidade as i16,
            vaga.horas_p_semana as i16
        )
        .execute(&mut *self.db)
        .await
        .unwrap_or_else(|err| panic!("{err}"));
    }

    pub async fn buscar_vaga_por_id(&mut self, id: &Uuid) -> Option<Vaga> {
        let query = format!("{BASE_SELECT_QUERY}  WHERE v.id = $1");

        sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(&mut *self.db)
            .await
            .unwrap_or_else(|err| panic!("{err}"))
    }

    pub async fn buscar_vagas(&mut self) -> Vec<Vaga> {
        sqlx::query_as(BASE_SELECT_QUERY)
            .fetch_all(&mut *self.db)
            .await
            .unwrap_or_else(|err| panic!("{err}"))
    }
}

/*  vaga (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    id_projeto          UUID            NOT NULL,
    titulo_customizado  VARCHAR(200),
    detalhes            TEXT            NOT NULL,
    quantidade          SMALLINT        NOT NULL,
    horas_p_semana      SMALLINT        NOT NULL,
)
 */
