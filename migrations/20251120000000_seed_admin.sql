-- Seed default admin user
INSERT INTO users (email, password_hash, role)
VALUES ('admin@alpha3d.xyz', '$argon2id$v=19$m=19456,t=2,p=1$ntSeX23DOlc7PwByxs2ocQ$PMNXuB5N5ippY5xOo3R1sORsOM6Z+WgmYMz78f3Zk7Y', 'ADMIN')
ON CONFLICT (email) DO NOTHING;
