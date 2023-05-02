# Burnout-detector 🧠🧑‍🚒

This toy aim to detect your idle activity and invite you to take a break and do some gymnastics.

---

**Disclaimer**

This gadget is not able to detect a real burnout! Please take care of yourself.

---

This is compatible with _waybar_ as a custom module, run it with the `--waybar` parameter to micmic the [waybar-eyes](https://github.com/cyrinux/waybar-eyes) module.
It will display an eye per 5min and will notify you by popup when the max eyes number is reach.

## Configuration

### Waybar

~/.config/waybar/config

```
...
    "custom/burnout-detector": {
        "exec": "burnout-detector --waybar --max-active-sessions 3",
        "return-type": "json",
    },
...
```

There is also 3 class you can use `ok`, `warning` and `critical`.

### Resources

The exercices are inspired (copied actually) from the [SafeEyes project](https://github.com/slgobinath/SafeEyes/blob/master/safeeyes/config/safeeyes.json#L20).
