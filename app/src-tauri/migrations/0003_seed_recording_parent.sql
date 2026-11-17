-- Restore the example recording's documented place in the wiki tree for existing workspaces.
UPDATE objects SET parent_id='getting-started' WHERE id='orientation-recording' AND parent_id IS NULL AND EXISTS (SELECT 1 FROM objects WHERE id='getting-started');
