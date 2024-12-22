resource "cloudflare_zero_trust_access_application" "site_preview" {
  zone_id                   = data.cloudflare_zone.site.id
  name                      = cloudflare_pages_project.site.name
  domain                    = cloudflare_pages_domain.site_dev.domain
  type                      = "self_hosted"
  session_duration          = "720h"
  auto_redirect_to_identity = true
  app_launcher_visible      = false

  policies = []
}

resource "cloudflare_zero_trust_access_policy" "site_preview" {
  zone_id        = data.cloudflare_zone.site.id
  application_id = cloudflare_zero_trust_access_application.site_preview.id
  name           = cloudflare_pages_project.site.name
  decision       = "allow"
  precedence     = 1

  include {
    email = var.cloudflare_access_emails
  }

  require {
    email = var.cloudflare_access_emails
  }
}

# Typically, creating the redirect list would not be enough; you would also
# need to create the bulk redirect rule as well.
#
# However, Cloudflare seems to requires that all bulk redirect rules be
# contained within a single ruleset, which is a problem for us because this
# project shares a Cloudflare account with other projects, each having their
# own bulk redirect rules.
#
# So, that logic is being handled elsewhere, in another codebase, and we don't
# need to worry about it here.
#
# If you're trying to deploy your own instance of this app, be aware that
# you'll need to create the bulk redirect rule yourself.
resource "cloudflare_list" "pages_dev_domains" {
  account_id  = var.cloudflare_account_id
  kind        = "redirect"
  name        = "notwithouthelp_pages_dev_domains"
  description = "List of *.notwithouthelp.pages.dev domains"

  item {
    value {
      redirect {
        source_url            = "${cloudflare_pages_project.site.subdomain}/"
        target_url            = "https://${cloudflare_pages_domain.site.domain}"
        status_code           = 301
        include_subdomains    = "enabled"
        preserve_query_string = "enabled"
        subpath_matching      = "enabled"
        preserve_path_suffix  = "enabled"
      }
    }
  }
}
