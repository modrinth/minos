local claims = {
  email_verified: true,
} + std.extVar('claims');

{
  identity: {
    traits: {
      [if 'email' in claims && claims.email_verified then 'email' else null]: claims.email,
      username: claims.name
    },
    metadata_public: {
      google_id: claims.sub      
    }
  },
}


