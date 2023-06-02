local claims = std.extVar('claims');
{
  identity: {
    traits: {
      // Allowing unverified email addresses enables account
      // enumeration attacks,  if the value is used for
      // verification or as a password login identifier.
      //
      // If connecting only to your organization (one tenant), claims.email is safe to use
      // if you haven't actively disabled e-mail verification during sign-up.
      //
      // The email might be empty if the account isn't linked to an email address.
      // For a human readable identifier, consider using the "preferred_username" claim.
      [if 'email' in claims then 'email' else null]: claims.email,
      claims.username : preferred_username,
      [if 'name' in claims then 'name' else null]: claims.name
    },
    metadata_public: {
      microsoft_id: claims.oid,
      [if "picture" in claims then "default_picture" else null]: claims.picture
    }
  },
}
