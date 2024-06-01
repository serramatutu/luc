```yaml
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

# Fetch all posts
This request fetches all posts from the server.

