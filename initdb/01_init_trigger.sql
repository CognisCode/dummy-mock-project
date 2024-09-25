CREATE OR REPLACE FUNCTION limit_rows() 
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the count exceeds 5000
    IF (SELECT COUNT(*) FROM simulation_data) >= 5000 THEN
        -- Delete the oldest row
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

-- Create the trigger that calls the function on every insert
CREATE TRIGGER row_limit_trigger
AFTER INSERT ON simulation_data
FOR EACH ROW EXECUTE FUNCTION limit_rows();
