resource "cloudflare_zero_trust_access_application" "site_preview" {
  zone_id                   = data.cloudflare_zone.site.id
  name                      = cloudflare_pages_project.site.name
  domain                    = cloudflare_pages_domain.site_dev.domain
  type                      = "self_hosted"
  session_duration          = "720h"
  auto_redirect_to_identity = true

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

resource "cloudflare_ruleset" "redirect_pages_dev_domains" {
  account_id  = var.cloudflare_account_id
  name        = "redirect *.notwithouthelp.pages.dev domains"
  description = "redirect *.notwithouthelp.pages.dev domains"
  kind        = "root"
  phase       = "http_request_redirect"

  rules {
    action = "redirect"

    action_parameters {
      from_list {
        name = cloudflare_list.pages_dev_domains.name
        key  = "http.request.full_uri"
      }
    }

    expression  = "http.request.full_uri in ${format("$%s", cloudflare_list.pages_dev_domains.name)}"
    description = "Apply redirects from ${cloudflare_list.pages_dev_domains.name}"
    enabled     = true
  }
}
