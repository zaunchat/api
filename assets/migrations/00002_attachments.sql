ALTER TABLE attachments
  DROP COLUMN uploader_id,
  ADD COLUMN content_type VARCHAR(64) NOT NULL;