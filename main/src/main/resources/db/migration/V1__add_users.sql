CREATE TABLE account (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY NOT NULL,
   username VARCHAR(255) NOT NULL,
   password VARCHAR(255) NOT NULL,
   CONSTRAINT pk_account PRIMARY KEY (id)
);

ALTER TABLE account ADD CONSTRAINT uc_account_username UNIQUE (username);

CREATE TABLE user_role (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY NOT NULL,
   authority VARCHAR(255) NOT NULL,
   CONSTRAINT pk_userrole PRIMARY KEY (id)
);

CREATE TABLE user_roles (
  user_id BIGINT NOT NULL,
   user_role_id BIGINT NOT NULL,
   CONSTRAINT pk_user_roles PRIMARY KEY (user_id, user_role_id)
);

ALTER TABLE user_roles ADD CONSTRAINT fk_userol_on_account FOREIGN KEY (user_id) REFERENCES account (id);
ALTER TABLE user_roles ADD CONSTRAINT fk_userol_on_user_role FOREIGN KEY (user_role_id) REFERENCES user_role (id);