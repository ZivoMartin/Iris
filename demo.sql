RESET;
CREATE TABLE Humain(id INT PRIMARY KEY, the_name STRING DEFAULT 'Clown', age INT, vivant BOOL DEFAULT 1);
INSERT INTO Humain (id, the_name, age) VALUES (1, 'Joah', 20);
INSERT INTO Humain (id, the_name, age) VALUES (2, 'Martin', 19);
INSERT INTO Humain (id, the_name, age) VALUES (3, 'Raghid', 17);
INSERT INTO Humain (id, the_name, age) VALUES (2, 'Dabi', 18);
INSERT INTO Humain (id, the_name, age) VALUES (5, 'Vico', 18);
SELECT age, the_name FROM Humain WHERE age!=17;
SET age=17 FROM Humain WHERE the_name == 'Joah';
SELECT * FROM Humain WHERE age>18;
DELETE FROM Humain WHERE the_name=='Raghid'; 
SELECT * FROM Humain WHERE age==17;

