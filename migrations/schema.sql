-- Add migration script here
-- Table personne (équivalent de la structure Personne)
CREATE TABLE personne (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    use_name VARCHAR(50) NOT NULL,
    numero VARCHAR(15) NOT NULL
);

-- Table cours (équivalent de la structure Cours)
CREATE TABLE cours (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(100) NOT NULL,
    description TEXT,
    duree VARCHAR(20),
    categorie VARCHAR(50),
    prix DECIMAL(10, 2),
    prix_promo DECIMAL(10, 2),
    langue VARCHAR(20) NOT NULL
);
