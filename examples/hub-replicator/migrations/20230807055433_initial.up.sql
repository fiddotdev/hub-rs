CREATE TABLE hubSubscriptions (
                                  host TEXT NOT NULL PRIMARY KEY,
                                  last_event_id BIGINT
);

CREATE TABLE messages (
                          id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                          createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                          updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                          deletedAt TIMESTAMP,
                          prunedAt TIMESTAMP,
                          revokedAt TIMESTAMP,
                          timestamp TIMESTAMP NOT NULL,
                          messageType SMALLINT NOT NULL,
                          fid BIGINT NOT NULL,
                          hash BYTEA NOT NULL,
                          hashScheme SMALLINT NOT NULL,
                          signature BYTEA NOT NULL,
                          signatureScheme SMALLINT NOT NULL,
                          signer BYTEA NOT NULL,
                          raw BYTEA NOT NULL,
                          CONSTRAINT messages_hash_unique UNIQUE (hash)
);

CREATE INDEX messages_timestamp_index ON messages (timestamp);

CREATE TABLE casts (
                       id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                       createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                       updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                       deletedAt TIMESTAMP,
                       timestamp TIMESTAMP NOT NULL,
                       fid BIGINT NOT NULL,
                       hash BYTEA NOT NULL,
                       parentHash BYTEA,
                       parentFid BIGINT,
                       parentUrl TEXT,
                       text TEXT NOT NULL,
                       embeds JSONB NOT NULL DEFAULT '{}',
                       mentions BIGINT[] NOT NULL DEFAULT '{}',
                       mentionsPositions SMALLINT[] NOT NULL DEFAULT '{}',
                       CONSTRAINT casts_hash_unique UNIQUE (hash),
                       CONSTRAINT casts_hash_foreign FOREIGN KEY (hash) REFERENCES messages (hash)
);

CREATE INDEX casts_fid_timestamp_index ON casts (fid, timestamp);

CREATE INDEX casts_timestamp_index ON casts (timestamp);

CREATE INDEX casts_parent_hash_parent_fid_index ON casts (parentHash, parentFid) WHERE parentHash IS NOT NULL AND parentFid IS NOT NULL;

CREATE INDEX casts_parent_url_index ON casts (parentUrl) WHERE parentUrl IS NOT NULL;

CREATE TABLE reactions (
                           id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                           createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                           updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                           deletedAt TIMESTAMP,
                           timestamp TIMESTAMP NOT NULL,
                           reactionType SMALLINT NOT NULL,
                           fid BIGINT NOT NULL,
                           hash BYTEA NOT NULL,
                           targetHash BYTEA,
                           targetFid BIGINT,
                           targetUrl TEXT,
                           CONSTRAINT reactions_hash_unique UNIQUE (hash),
                           CONSTRAINT reactions_hash_foreign FOREIGN KEY (hash) REFERENCES messages (hash)
);

CREATE INDEX reactions_fid_timestamp_index ON reactions (fid, timestamp);

CREATE INDEX reactions_target_hash_target_fid_index ON reactions (targetHash, targetFid) WHERE targetHash IS NOT NULL AND targetFid IS NOT NULL;

CREATE INDEX reactions_target_url_index ON reactions (targetUrl) WHERE targetUrl IS NOT NULL;

CREATE TABLE signers (
                         id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                         createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                         updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                         deletedAt TIMESTAMP,
                         timestamp TIMESTAMP NOT NULL,
                         fid BIGINT NOT NULL,
                         hash BYTEA NOT NULL,
                         custodyAddress BYTEA NOT NULL,
                         signer BYTEA NOT NULL,
                         name TEXT,
                         CONSTRAINT signers_hash_unique UNIQUE (hash),
                         CONSTRAINT signers_hash_foreign FOREIGN KEY (hash) REFERENCES messages (hash)
);

CREATE INDEX signers_fid_timestamp_index ON signers (fid, timestamp);

CREATE TABLE verifications (
                               id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                               createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                               updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                               deletedAt TIMESTAMP,
                               timestamp TIMESTAMP NOT NULL,
                               fid BIGINT NOT NULL,
                               hash BYTEA NOT NULL,
                               claim JSONB NOT NULL,
                               CONSTRAINT verifications_hash_unique UNIQUE (hash),
                               CONSTRAINT verifications_hash_foreign FOREIGN KEY (hash) REFERENCES messages (hash)
);

CREATE INDEX verifications_claim_address_index ON verifications ((claim ->> 'address'::text));

CREATE INDEX verifications_fid_timestamp_index ON verifications (fid, timestamp);

CREATE TABLE userData (
                          id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                          createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                          updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                          deletedAt TIMESTAMP,
                          timestamp TIMESTAMP NOT NULL,
                          fid BIGINT NOT NULL,
                          hash BYTEA NOT NULL,
                          type SMALLINT NOT NULL,
                          value TEXT NOT NULL,
                          CONSTRAINT user_data_hash_unique UNIQUE (hash),
                          CONSTRAINT user_data_fid_type_unique UNIQUE (fid, type),
                          CONSTRAINT user_data_hash_foreign FOREIGN KEY (hash) REFERENCES messages (hash)
);

CREATE INDEX user_data_fid_index ON userData (fid);

CREATE TABLE fids (
                      fid BIGINT PRIMARY KEY,
                      createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                      updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                      custodyAddress BYTEA NOT NULL
);

CREATE TABLE fnames (
                        fname TEXT PRIMARY KEY,
                        createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                        updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                        custodyAddress BYTEA NOT NULL,
                        expiresAt TIMESTAMP NOT NULL
);

CREATE TABLE links (
                       id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                       fid BIGINT,
                       targetFid BIGINT,
                       hash BYTEA NOT NULL,
                       timestamp TIMESTAMP NOT NULL,
                       createdAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                       updatedAt TIMESTAMP NOT NULL DEFAULT current_timestamp,
                       deletedAt TIMESTAMP,
                       type TEXT,
                       displayTimestamp TIMESTAMP,
                       CONSTRAINT links_hash_unique UNIQUE (hash),
                       CONSTRAINT links_fid_target_fid_type_unique UNIQUE (fid, targetFid, type)
);
