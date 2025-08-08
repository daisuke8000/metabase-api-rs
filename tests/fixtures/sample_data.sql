-- Sample data for integration testing
-- This creates a simple e-commerce schema with test data

-- Create tables
CREATE TABLE IF NOT EXISTS customers (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    category VARCHAR(100),
    price DECIMAL(10, 2) NOT NULL,
    stock_quantity INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customers(id),
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(50) DEFAULT 'pending',
    total_amount DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    product_id INTEGER REFERENCES products(id),
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    line_total DECIMAL(10, 2) NOT NULL
);

-- Insert sample customers
INSERT INTO customers (email, name) VALUES
('alice@example.com', 'Alice Johnson'),
('bob@example.com', 'Bob Smith'),
('charlie@example.com', 'Charlie Brown'),
('diana@example.com', 'Diana Prince'),
('eve@example.com', 'Eve Wilson');

-- Insert sample products
INSERT INTO products (name, category, price, stock_quantity) VALUES
('Laptop', 'Electronics', 999.99, 50),
('Mouse', 'Electronics', 29.99, 200),
('Keyboard', 'Electronics', 79.99, 150),
('Monitor', 'Electronics', 299.99, 75),
('Desk Chair', 'Furniture', 199.99, 30),
('Standing Desk', 'Furniture', 599.99, 20),
('Notebook', 'Stationery', 4.99, 500),
('Pen Set', 'Stationery', 19.99, 300),
('Coffee Mug', 'Kitchen', 12.99, 100),
('Water Bottle', 'Kitchen', 24.99, 80);

-- Insert sample orders
INSERT INTO orders (customer_id, order_date, status, total_amount) VALUES
(1, '2024-01-01 10:00:00', 'completed', 1029.98),
(2, '2024-01-02 11:30:00', 'completed', 329.97),
(3, '2024-01-03 14:15:00', 'completed', 799.98),
(1, '2024-01-04 09:45:00', 'completed', 224.97),
(4, '2024-01-05 16:20:00', 'shipped', 1299.97),
(5, '2024-01-06 13:00:00', 'processing', 37.98),
(2, '2024-01-07 10:30:00', 'pending', 604.98);

-- Insert sample order items
INSERT INTO order_items (order_id, product_id, quantity, unit_price, line_total) VALUES
(1, 1, 1, 999.99, 999.99),
(1, 2, 1, 29.99, 29.99),
(2, 3, 1, 79.99, 79.99),
(2, 4, 1, 299.99, 299.99),
(3, 5, 2, 199.99, 399.98),
(3, 6, 1, 599.99, 599.99),
(4, 7, 5, 4.99, 24.95),
(4, 8, 10, 19.99, 199.95),
(5, 1, 1, 999.99, 999.99),
(5, 4, 1, 299.99, 299.99),
(6, 9, 2, 12.99, 25.98),
(6, 10, 1, 24.99, 24.99),
(7, 6, 1, 599.99, 599.99),
(7, 7, 1, 4.99, 4.99);

-- Create useful views for testing
CREATE VIEW order_summary AS
SELECT 
    o.id as order_id,
    c.name as customer_name,
    c.email as customer_email,
    o.order_date,
    o.status,
    o.total_amount,
    COUNT(oi.id) as item_count
FROM orders o
JOIN customers c ON o.customer_id = c.id
LEFT JOIN order_items oi ON o.id = oi.order_id
GROUP BY o.id, c.name, c.email, o.order_date, o.status, o.total_amount;

CREATE VIEW product_sales AS
SELECT 
    p.id as product_id,
    p.name as product_name,
    p.category,
    p.price as current_price,
    COUNT(oi.id) as times_sold,
    SUM(oi.quantity) as total_quantity_sold,
    SUM(oi.line_total) as total_revenue
FROM products p
LEFT JOIN order_items oi ON p.id = oi.product_id
GROUP BY p.id, p.name, p.category, p.price;