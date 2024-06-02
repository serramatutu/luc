# Fetch all posts
This request fetches all posts from the server.

The following code will not be run by `luc` since it is untagged
```yaml
foo: bar
```

## Request definition
```yaml luc
api: luc.api.http_request.HttpRequestBuilder
spec:
  request:
    method: GET
    url: {{ env.BASE_URL }}/posts/
    headers:
      x-custom-header: My header value
  hooks:
    before: python all.py
    after: python all.py
```

