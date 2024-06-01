import json

def before(hook):
    res = {
        "api": "luc.spec.v1.BeforeHook.Response",
    }
    return res

def after(hook):
    res = {
        "api": "luc.spec.v1.AfterHook.Response",
    }
    return res

def main():
    hook_str = input() 
    hook = json.loads(hook_str)

    hook_map = {
        "luc.spec.v1.BeforeHook.Request": before,
        "luc.spec.v1.AfterHook.Request": after,
    }
    hook_handler = hook_map.get(hook["api"], None)

    if hook_handler is None:
        return

    response = hook_handler(hook)
    print(json.dumps(response))

if __name__ == "__main__":
    main()
