# Tool Calls in the UI

To get tool calls to work you need a host that will detect the model is trying to call a tool and make the tool call and return the results to the model.

Let's enable a tool in Bionic.

![Alt text](/architect-course/020-basics-of-tool-calls/010-basics-of-tool-calls/030-tool-calls-in-the-ui/enable-datetime.png "MCP Servers")

## Prompt for a date

```
What's the date?
```

![Alt text](/architect-course/020-basics-of-tool-calls/010-basics-of-tool-calls/030-tool-calls-in-the-ui/datetime-results.png "MCP Servers")

## The Workflow Model -> Tool Call -> Bionic -> Results

If you click on the `+` after the tool call, you'll see what the model sent to Bionic and what Bionic then returned to the model.

![Alt text](/architect-course/020-basics-of-tool-calls/010-basics-of-tool-calls/030-tool-calls-in-the-ui/debug-tool.png "MCP Servers")
