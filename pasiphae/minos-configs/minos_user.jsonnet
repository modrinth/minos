function(ctx) {
  id : ctx.identity.id,
  username: ctx.identity.traits.username,
  email : ctx.identity.traits.email,
  name: if "name" in ctx.identity.traits then ctx.identity.traits.name else null,
  default_picture: if ctx.identity.metadata_public != null && "default_picture" in ctx.identity.metadata_public then ctx.identity.metadata_public.default_picture else null,
  github_id: if ctx.identity.metadata_public != null && "github_id" in ctx.identity.metadata_public then ctx.identity.metadata_public.github_id else null,
  discord_id: if ctx.identity.metadata_public != null && "discord_id" in ctx.identity.metadata_public then ctx.identity.metadata_public.discord_id else null,
  google_id: if ctx.identity.metadata_public != null && "google_id" in ctx.identity.metadata_public then ctx.identity.metadata_public.google_id else null,
  gitlab_id: if ctx.identity.metadata_public != null && "gitlab_id" in ctx.identity.metadata_public then ctx.identity.metadata_public.gitlab_id else null,
  microsoft_id: if ctx.identity.metadata_public != null && "microsoft_id" in ctx.identity.metadata_public then ctx.identity.metadata_public.microsoft_id else null,
  apple_id: if ctx.identity.metadata_public != null && "apple_id" in ctx.identity.metadata_public then ctx.identity.metadata_public.apple_id else null,
}
