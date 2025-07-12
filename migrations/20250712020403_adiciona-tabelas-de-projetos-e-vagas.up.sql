-- Add up migration script here
CREATE TYPE tipo_coord AS ENUM ('coord', 'vice');

CREATE TABLE projeto (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    title               VARCHAR(200)    NOT NULL,
    content             TEXT            NOT NULL,
    created_at          TIMESTAMP       NOT NULL    DEFAULT now(),

    CONSTRAINT projeto_pk PRIMARY KEY (id)
);

CREATE TABLE projeto_coordenadores (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    id_projeto          UUID            NOT NULL,
    id_coord            UUID            NOT NULL,
    tipo_coord          tipo_coord      NOT NULL,

    CONSTRAINT proj_coord_pk    PRIMARY KEY (id),
    CONSTRAINT projeto_coord_projeto_fk FOREIGN KEY (id_projeto) REFERENCES projeto(id) ON DELETE cascade,
    CONSTRAINT projeto_coord_coord_fk   FOREIGN KEY (id_coord) REFERENCES usuario(id) ON DELETE cascade
);

CREATE TABLE vaga (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    id_projeto          UUID            NOT NULL,
    titulo_customizado  VARCHAR(200),
    detalhes            TEXT            NOT NULL,
    quantidade          SMALLINT        NOT NULL,
    horas_p_semana      SMALLINT        NOT NULL,

    CONSTRAINT vaga_pk PRIMARY KEY (id),
    CONSTRAINT vaga_projeto_fk   FOREIGN KEY (id_projeto) REFERENCES projeto(id) ON DELETE cascade
);