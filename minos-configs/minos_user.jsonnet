function(ctx) {
  id : ctx.identity.id,
  username: ctx.identity.traits.username,
  email : ctx.identity.traits.email,
  [if "default_picture" in ctx.identity.metadata_public then "default_picture" else null]: ctx.identity.metadata_public.default_picture,

  [if "name" in ctx.identity.traits then "name" else null]:  ctx.identity.traits.name,
  [if "github_id" in ctx.identity.metadata_public then "github_id" else null]: ctx.identity.metadata_public.github_id,
  [if "discord_id" in ctx.identity.metadata_public then "discord_id" else null]: ctx.identity.metadata_public.discord_id,
  [if "google_id" in ctx.identity.metadata_public then "google_id" else null]: ctx.identity.metadata_public.google_id,
  [if "gitlab_id" in ctx.identity.metadata_public then "gitlab_id" else null]: ctx.identity.metadata_public.gitlab_id,
  [if "microsoft_id" in ctx.identity.metadata_public then "microsoft_id" else null]: ctx.identity.metadata_public.microsoft_id,
  [if "apple_id" in ctx.identity.metadata_public then "apple_id" else null]: ctx.identity.metadata_public.apple_id,
}
