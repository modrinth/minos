local claims = {
  email_verified: false
} + std.extVar('claims');

{
  identity: {
    traits: {
      // Allowing unverified email addresses enables account
      // enumeration attacks, especially if the value is used for
      // e.g. verification or as a password login identifier.
      //
      // Therefore we only return the email if it (a) exists and (b) is marked verified
      // by Gitlab.
      [if "email" in claims && claims.email_verified then "email" else null]: claims.email,
      "username": claims.preferred_username,
      [if "name" in claims then "name" else null]: claims.name
    },
    metadata_public: {
      "gitlab_id": claims.sub,
      [if "picture" in claims then "default_picture" else null]: claims.picture
    }
  },
}


