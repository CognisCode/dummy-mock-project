-- allows for a maximum of 5000 rows 0-4999
CREATE OR REPLACE FUNCTION limit_rows() 
RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT COUNT(*) FROM simulation_data) >= 5000 THEN
        DELETE FROM simulation_data 
        WHERE id IN (
            SELECT id FROM simulation_data 
            ORDER BY timestamp 
            LIMIT 1
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER row_limit_trigger
AFTER INSERT ON simulation_data
FOR EACH ROW EXECUTE FUNCTION limit_rows();
