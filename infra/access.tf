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

# This is defined in a Terraform module from a different unrelated project; we
# only have one of these for the whole account.
#
# That project defines a bulk redirect rule for redirecting the *.pages.dev
# domains to the main domain.
#
# If you're trying to deploy your own instance of this app, be aware that
# you'll need to create that bulk redirect rule yourself.
data "cloudflare_list" "pages_dev_domains" {
  account_id = var.cloudflare_account_id
  name       = "pages_dev_domains"
}

resource "cloudflare_list_item" "site_pages_dev_domain" {
  account_id = var.cloudflare_account_id
  list_id    = data.cloudflare_list.pages_dev_domains.id

  redirect {
    source_url            = "${cloudflare_pages_project.site.subdomain}/"
    target_url            = "https://${cloudflare_pages_domain.site.domain}"
    status_code           = 301
    include_subdomains    = true
    preserve_query_string = true
    subpath_matching      = true
    preserve_path_suffix  = true
  }
}
