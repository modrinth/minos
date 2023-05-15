local claims = {
  email_verified: true,
} + std.extVar('claims');

{
  identity: {
    traits: {
      [if 'email' in claims && claims.email_verified then 'email' else null]: claims.email,
      username: claims.preferred_username,
      [if 'name' in claims then 'name' else null]: claims.name
    },
    metadata_public: {
      google_id: claims.sub,
      [if "picture" in claims then "default_picture" else null]: claims.picture
    }
  },
}


