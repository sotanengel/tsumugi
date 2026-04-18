import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";

const server = new McpServer({
  name: "tsumugi",
  version: "0.1.0",
});

// Phase 3: Register tools here
// - create_project
// - import_media
// - add_clip
// - trim_clip
// - add_text
// - set_transition
// - apply_effect
// - generate_subtitles
// - get_timeline
// - render

server.tool("get_timeline", "Get the current timeline state", {}, async () => {
  return {
    content: [
      {
        type: "text" as const,
        text: JSON.stringify({ status: "not_implemented", message: "Phase 3" }),
      },
    ],
  };
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
}

main().catch(console.error);
