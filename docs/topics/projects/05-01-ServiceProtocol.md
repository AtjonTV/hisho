# ServiceProtocol

The Protocol defines how Hisho will try to connect to the given URI.

Multiple protocols may use `tcp://` or `udp://` for URIs.  
While `tcp://cloudflware:443` is a technically valid URI for an HTTP client to consume, `reqwest` does not seem to be happy with it.

| Name | Description                                                                  |
|------|------------------------------------------------------------------------------|
| HTTP | Use HTTP/HTTPS; A HTTP `GET` Request must respond with `200 OK?` for success |
| TCP  | Use raw TCP; A TCP connection must be established for success                |

