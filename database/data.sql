-- ==========================
-- COMPANIES (10 rows)
-- ==========================
INSERT INTO companys (
    company_id, company_name, company_street, company_city, company_zipcode, company_country,
    company_phone, company_email, company_website
) VALUES
('6f1c1b60-67e7-4f1b-8358-3b73d6a1f2a1', 'TechNed B.V.', 'Keizersgracht 123', 'Amsterdam', '1015CJ', 'Netherlands', '+31 20 123 4567', 'info@techned.nl', 'https://www.techned.nl'),
('c9a4d3b4-2d5f-4b7d-a2f7-8479d1117b93', 'GlobalSoft Ltd.', '221B Baker Street', 'London', 'NW1 6XE', 'United Kingdom', '+44 20 7946 0991', 'contact@globalsoft.co.uk', 'https://www.globalsoft.co.uk'),
('f84c2b9e-9b1e-498a-9d6c-b15726aeb5af', 'DigiWare GmbH', 'Musterstraße 45', 'Berlin', '10115', 'Germany', '+49 30 1234567', 'support@digiware.de', 'https://www.digiware.de'),
('1d4aee77-89be-4e4c-bcbb-7f2326a6d71d', 'InnoTech NV', 'Stationsstraat 89', 'Utrecht', '3511CE', 'Netherlands', '+31 30 456 7890', 'hello@innotech.nl', 'https://www.innotech.nl'),
('3c582c8d-fc45-441f-a9ed-9a508f09f0f2', 'BlueSky Inc.', '500 Market St', 'San Francisco', '94105', 'United States', '+1 415 123 7890', 'info@bluesky.com', 'https://www.bluesky.com'),
('e4fdb978-3c2a-4c2e-bc19-3c1df6b43ef4', 'CloudOne S.A.', 'Rue de la Loi 155', 'Brussels', '1040', 'Belgium', '+32 2 289 1234', 'contact@cloudone.be', 'https://www.cloudone.be'),
('64b417b1-4f6f-482f-bb8c-4a6187b7299a', 'EcoLogix B.V.', 'Nieuwezijds Voorburgwal 182', 'Amsterdam', '1012SJ', 'Netherlands', '+31 20 567 1234', 'info@ecologix.nl', 'https://www.ecologix.nl'),
('d94b7a77-bc4e-4f8f-8020-1e1d4d7de1f1', 'NextWave S.L.', 'Calle de Alcalá 45', 'Madrid', '28014', 'Spain', '+34 91 123 4567', 'hello@nextwave.es', 'https://www.nextwave.es'),
('a4a7f4b3-13d3-4b39-9e24-4fcd6c8e728e', 'Nordic Solutions AB', 'Kungsgatan 12', 'Stockholm', '111 35', 'Sweden', '+46 8 123 456', 'info@nordicsolutions.se', 'https://www.nordicsolutions.se'),
('2e9ad35c-dc66-4b0d-a5bb-2d3ff9e0a8f2', 'OrangeBits SARL', '5 Avenue Victor Hugo', 'Paris', '75016', 'France', '+33 1 44 55 66 77', 'support@orangebits.fr', 'https://www.orangebits.fr');

-- ==========================
-- COMPANY CONTACTS (10 rows)
-- ==========================
INSERT INTO company_contacts (
    contact_id, company, firstname, lastname, contact_email, contact_phone, contact_mobile, contact_titel
) VALUES
('c4a9f5de-d8a0-4f7e-9bcd-dc96c75e8fd1', '6f1c1b60-67e7-4f1b-8358-3b73d6a1f2a1', 'Jan', 'de Vries', 'jan.vries@techned.nl', '+31 20 123 4567', '+31 6 1234 5678', 'CTO'),
('56f3acde-5dbd-4c54-a6d0-5d2b9f7c12f9', 'c9a4d3b4-2d5f-4b7d-a2f7-8479d1117b93', 'Alice', 'Smith', 'alice.smith@globalsoft.co.uk', '+44 20 7946 0991', '+44 7400 123456', 'Project Manager'),
('bf43d92f-4b3d-4e25-9472-7c2ff0133d9a', 'f84c2b9e-9b1e-498a-9d6c-b15726aeb5af', 'Lars', 'Müller', 'lars.mueller@digiware.de', '+49 30 1234567', '+49 170 1234567', 'Product Owner'),
('d34244ea-4a1c-492f-9fd3-894824c0f347', '1d4aee77-89be-4e4c-bcbb-7f2326a6d71d', 'Sanne', 'Jansen', 's.jansen@innotech.nl', '+31 30 456 7890', NULL, 'CEO'),
('2f5e6aeb-41e4-40e1-b8aa-5c9a63a2c6a2', '3c582c8d-fc45-441f-a9ed-9a508f09f0f2', 'Michael', 'Johnson', 'm.johnson@bluesky.com', '+1 415 123 7890', '+1 650 555 7890', 'Sales Manager'),
('69ad9d5d-279f-4f22-9e23-79b32b3b3c01', 'e4fdb978-3c2a-4c2e-bc19-3c1df6b43ef4', 'Elise', 'Dubois', 'elise.dubois@cloudone.be', '+32 2 289 1234', '+32 485 123456', 'Support Lead'),
('b1c8ef45-3f82-4a14-9e55-40d9f8cc7340', '64b417b1-4f6f-482f-bb8c-4a6187b7299a', 'Peter', 'van Dijk', 'p.vandijk@ecologix.nl', '+31 20 567 1234', NULL, 'Engineer'),
('af02ef7a-7f39-4d20-88d4-6797bc8f40d2', 'd94b7a77-bc4e-4f8f-8020-1e1d4d7de1f1', 'Carlos', 'García', 'carlos.garcia@nextwave.es', '+34 91 123 4567', '+34 600 123 456', 'Consultant'),
('1e0b1b29-88db-4c14-a9df-9d67ff70f505', 'a4a7f4b3-13d3-4b39-9e24-4fcd6c8e728e', 'Anna', 'Eriksson', 'anna.eriksson@nordicsolutions.se', '+46 8 123 456', NULL, 'HR Manager'),
('73f6c428-d4f2-4786-89b8-f10a73127401', '2e9ad35c-dc66-4b0d-a5bb-2d3ff9e0a8f2', 'Luc', 'Moreau', 'luc.moreau@orangebits.fr', '+33 1 44 55 66 77', '+33 6 12 34 56 78', 'Marketing Director');

-- ==========================
-- USER STATUS TYPES (5 rows are enough, reusing your original)
-- ==========================
INSERT INTO user_status_types (status_name) VALUES
('Active'),
('Inactive'),
('Suspended'),
('Pending Verification'),
('Archived');

-- ==========================
-- ROLES (10 rows)
-- ==========================
INSERT INTO roles (
    role_id, role_name, create_orders, create_quotations, create_purchase_orders, create_articles,
    create_customers, create_suppliers, create_users, create_companys
) VALUES
('1a4c1f7b-2a9e-4df0-a6e1-7c5fd5e9a3a1', 'Administrator', TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE),
('23e1df45-3c8b-4a9d-a4a5-8a92c12f0e23', 'Sales Manager', TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE),
('bd4124e9-6d4c-4f6f-bf10-5d54a02e86e3', 'Procurement Officer', FALSE, FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE),
('5b2a9f38-0c23-43a4-bc88-8d3f1bb382f4', 'Product Manager', FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE),
('efb3b12c-d5f7-4e28-bbb4-4a5d1389a9c9', 'Customer Support', FALSE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE),
('3af41d87-5b34-44cb-9a17-6e5c64d62c90', 'Finance Admin', TRUE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE),
('de17ac90-892b-4cf3-a67b-6d7c2b8d3a10', 'Warehouse Staff', FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, FALSE),
('9b3ed54c-6a19-47fd-a93a-7ef5e1c3b5f0', 'HR Manager', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, FALSE),
('c24adf11-2b9c-498d-9f4c-3a1cfb2d4e10', 'IT Admin', FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, TRUE),
('7d4a1e35-19c4-4a68-89c2-7e6b21f4c7a0', 'Sales Representative', TRUE, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE);

-- ==========================
-- USERS (10 rows)
-- ==========================
INSERT INTO users (
    user_id, password, email, role_id, first_name, last_name, status, last_interaction_time
) VALUES
('8a9d2c4f-5d2f-48b1-b9fc-7e5f6b2d8c10', 'pass001', 'jan.vandermeer@example.com', '1a4c1f7b-2a9e-4df0-a6e1-7c5fd5e9a3a1', 'Jan', 'van der Meer', (SELECT status_id FROM user_status_types WHERE status_name = 'Active'), '2025-07-20 08:15:00'),
('b3a4e51c-92f3-4c2e-b2f9-6d4b5c3e9f20', 'pass002', 'alice.smith@globalsoft.co.uk', '23e1df45-3c8b-4a9d-a4a5-8a92c12f0e23', 'Alice', 'Smith', (SELECT status_id FROM user_status_types WHERE status_name = 'Active'), '2025-07-21 10:00:00'),
('f8c4d13a-5d3f-4f9e-8f2a-9e4c6b2d7a30', 'pass003', 'lars.mueller@digiware.de', 'bd4124e9-6d4c-4f6f-bf10-5d54a02e86e3', 'Lars', 'Müller', (SELECT status_id FROM user_status_types WHERE status_name = 'Active'), '2025-07-21 10:00:00'),
('ad3e5b91-0d3c-4f2a-8c91-4f3d2a1b5c40', 'pass004', 'sanne.jansen@innotech.nl', '5b2a9f38-0c23-43a4-bc88-8d3f1bb382f4', 'Sanne', 'Jansen', (SELECT status_id FROM user_status_types WHERE status_name = 'Active'), '2025-07-22 09:45:00'),
('6e2c1f4a-7f3d-42c1-89b1-6d4c5a2f8d50', 'pass005', 'michael.johnson@bluesky.com', 'efb3b12c-d5f7-4e28-bbb4-4a5d1389a9c9', 'Michael', 'Johnson', (SELECT status_id FROM user_status_types WHERE status_name = 'Active'), '2025-07-21 15:30:00'),
('dd91b4e2-6c2f-4d3c-9a21-7f6c8b2d9e60', 'pass006', 'elise.dubois@cloudone.be', '3af41d87-5b34-44cb-9a17-6e5c64d62c90', 'Elise', 'Dubois', (SELECT status_id FROM user_status_types WHERE status_name = 'Active'), '2025-07-21 15:30:00'),
('cb1f3d7a-5c9a-4e1d-8b21-5f3c9e7a2b70', 'pass007', 'peter.vandijk@ecologix.nl', 'de17ac90-892b-4cf3-a67b-6d7c2b8d3a10', 'Peter', 'van Dijk', (SELECT status_id FROM user_status_types WHERE status_name = 'Suspended'), '2025-07-22 07:50:00'),
('ae4f3b2c-2a8d-4f5a-b9e1-7e3d5f9a1c80', 'pass008', 'carlos.garcia@nextwave.es', '9b3ed54c-6a19-47fd-a93a-7ef5e1c3b5f0', 'Carlos', 'García', (SELECT status_id FROM user_status_types WHERE status_name = 'Pending Verification'), '2025-07-22 07:50:00'),
('1f3c4d2e-9a5b-42f2-9e1f-7b6a4d2e3f90', 'pass009', 'anna.eriksson@nordicsolutions.se', 'c24adf11-2b9c-498d-9f4c-3a1cfb2d4e10', 'Anna', 'Eriksson', (SELECT status_id FROM user_status_types WHERE status_name = 'Archived'), '2025-07-19 16:10:00'),
('e5a7c1d3-0c2f-46a3-9e2d-4f5a7b2c6d00', 'pass010', 'luc.moreau@orangebits.fr', '7d4a1e35-19c4-4a68-89c2-7e6b21f4c7a0', 'Luc', 'Moreau', (SELECT status_id FROM user_status_types WHERE status_name = 'Suspended'), '2025-07-19 16:10:00');










