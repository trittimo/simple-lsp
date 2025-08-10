pub enum Either<A, B> {
    Left(A),
    Right(B)
}

pub struct Message {
    pub jsonrpc: String,
}

pub enum LSPAny {

}

pub enum LSPObject {

}

pub struct RequestMessage {
    pub id: i32,
    pub method: String,
    pub params: Option<Either<LSPObject, Vec<LSPAny>>>,
}

pub struct ResponseMessage {
    pub id: i32,
    pub result: Option<LSPAny>,
    pub error: Option<ResponseError>,
}

pub struct ResponseError {
    pub code: i32,
    pub message: String,
    pub data: Option<LSPAny>,
}

pub struct NotificationMessage {
    pub method: String,
    pub params: Option<Either<LSPObject, Vec<LSPAny>>>,
}

pub struct CancelParams {
    pub id: i32,
}

pub enum ProgressToken {
    Integer(i32),
    String(String)
}

pub struct ProgressParams {
    pub token: ProgressToken,
    pub value: LSPObject,
}

pub struct HoverParamsPosition {
    pub line: u32,
    pub character: u32,
}

pub struct HoverParams {
    pub text_document: String,
    pub position: HoverParamsPosition,
}

pub struct HoverResult {
    pub value: String,
}

pub struct Position {
    pub line: u32,
    pub character: u32,
}

pub struct Range {
    pub start: Position,
    pub end: Position,
}

pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

pub struct TextDocumentIdentifier {
    pub uri: String,
}

pub struct VersionedTextDocumentIdentifier {
    pub version: i32,
}

pub struct OptionalVersionedTextDocumentIdentifier {
    pub version: i32,
}

pub struct TextDocumentPositionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

pub struct Location {
    pub uri: String,
    pub range: Range,
}

pub struct LocationLink {
    pub origin_selection_range: Option<Range>,
    pub target_uri: String,
    pub target_range: Range,
    pub target_selection_range: Range,
}

pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<LSPAny>>,
}

pub struct InitializeParamsClientInfo {
    pub name: String,
    pub version: String,
}

pub enum TraceValue {
    Off,
    Messages,
    Verbose
}

pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String
}

pub struct InitializeParams {
    pub process_id: i32,
    pub client_info: Option<InitializeParamsClientInfo>,
    pub locale: Option<String>,
    pub root_path: Option<String>,
    pub root_uri: String,
    pub initialization_options: Option<LSPAny>,
    pub capabilities: ClientCapabilities,
    pub trace: Option<TraceValue>,
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

pub struct ClientCapabilitiesWorkspaceFileOperations {
    pub dynamic_registration: bool,
    pub did_create: bool,
    pub will_create: bool,
    pub did_rename: bool,
    pub will_rename: bool,
    pub did_delete: bool,
    pub will_delete: bool,
}

pub struct WorkspaceEditClientCapabilities {
    // TODO
}

pub struct DidChangeConfigurationClientCapabilities {
    // TODO
}

pub struct ExecuteCommandClientCapabilities {
    // TODO
}

pub struct SemanticTokensWorkspaceClientCapabilities {
    // TODO
}

pub struct CodeLensWorkspaceClientCapabilities {
    // TODO
}

pub struct InlineValueWorkspaceClientCapabilities {
    // TODO
}

pub struct InlayHintWorkspaceClientCapabilities {
    // TODO
}

pub struct DiagnosticWorkspaceClientCapabilities {
    // TODO
}

pub struct DidChangeWatchedFilesClientCapabilities {
    // TODO
}

pub struct ClientCapabilitiesWorkspace {
    pub apply_edit: bool,
    pub workspace_edit: WorkspaceEditClientCapabilities,
    pub did_change_configuration: DidChangeConfigurationClientCapabilities,
    pub did_change_watched_files: DidChangeWatchedFilesClientCapabilities,
    pub symbol: WorkspaceSymbolClientCapabilities,
    pub execute_command: ExecuteCommandClientCapabilities,
    pub workspace_folders: bool,
    pub configuration: bool,
    pub semantic_tokens: SemanticTokensWorkspaceClientCapabilities,
    pub code_lens: CodeLensWorkspaceClientCapabilities,
    pub file_operations: ClientCapabilitiesWorkspaceFileOperations,
    pub inline_value: InlineValueWorkspaceClientCapabilities,
    pub inlay_hint: InlayHintWorkspaceClientCapabilities,
    pub diagnostics: DiagnosticWorkspaceClientCapabilities,
}

pub struct ShowMessageRequestClientCapabilities {
    // TODO
}

pub struct ShowDocumentClientCapabilities {
    // TODO
}

pub struct ClientCapabilitiesWindow {
    pub work_done_progress: bool,
    pub show_message: ShowMessageRequestClientCapabilities,
    pub show_document: ShowDocumentClientCapabilities,
}

pub struct ClientCapabilitiesGeneralStaleRequestSupport {
    pub cancel: bool,
    pub retry_on_content_modified: Vec<String>,
}

pub struct RegularExpressionsClientCapabilities {
    // TODO
}

pub struct MarkdownClientCapabilities {
    // TODO
}

pub struct ClientCapabilitiesGeneral {
    pub stale_request_support: ClientCapabilitiesGeneralStaleRequestSupport,
    pub regular_expressions: RegularExpressionsClientCapabilities,
    pub markdown: MarkdownClientCapabilities,
    pub position_encodings: Vec<PositionEncodingKind>,
}

pub struct TextDocumentClientCapabilities {
    // TODO
}

pub struct NotebookDocumentClientCapabilities {
    // TODO
}

pub struct ClientCapabilities {
    pub workspace: Option<ClientCapabilitiesWorkspace>,
    pub text_document: Option<TextDocumentClientCapabilities>,
    pub notebook_document: Option<NotebookDocumentClientCapabilities>,
    pub window: Option<ClientCapabilitiesWindow>,
    pub general: Option<ClientCapabilitiesGeneral>,
    pub experimental: Option<LSPAny>,
}

pub struct InitializeResultServerInfo {
    pub name: String,
    pub version: String,
}

pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    pub server_info: Option<InitializeResultServerInfo>,
}

pub struct InitializeError {
    pub retry: bool,
}

pub struct FileOperationRegistrationOptions {
    // TODO
}

pub struct ServerCapabilitiesWorkspaceFileOperations {
    pub did_create: FileOperationRegistrationOptions,
    pub will_create: FileOperationRegistrationOptions,
    pub did_rename: FileOperationRegistrationOptions,
    pub will_rename: FileOperationRegistrationOptions,
    pub did_delete: FileOperationRegistrationOptions,
    pub will_delete: FileOperationRegistrationOptions,
}

pub struct WorkspaceFoldersServerCapabilities {
    // TODO
}

pub struct ServerCapabilitiesWorkspace {
    pub workspace_folders: WorkspaceFoldersServerCapabilities,
    pub file_operations: ServerCapabilitiesWorkspaceFileOperations,
}

pub struct PositionEncodingKind {
    // TODO
}

pub struct TextDocumentSyncOptions {
    // TODO
}

pub struct NotebookDocumentSyncOptions {
    // TODO
}

pub struct CompletionOptions {
    // TODO
}

pub struct SignatureHelpOptions {
    // TODO
}

pub struct CodeLensOptions {
    // TODO
}

pub struct DocumentLinkOptions {
    // TODO
}

pub struct DocumentOnTypeFormattingOptions {
    // TODO
}

pub struct ExecuteCommandOptions {
    // TODO
}

pub struct SemanticTokensOptions {
    // TODO
}

pub struct DiagnosticOptions {
    // TODO
}

pub struct ServerCapabilities {
    pub position_encoding: Option<PositionEncodingKind>,
    pub text_document_sync: Option<TextDocumentSyncOptions>,
    pub notebook_document_sync: Option<NotebookDocumentSyncOptions>,
    pub completion_provider: Option<CompletionOptions>,
    pub hover_provider: Option<bool>,
    pub signature_help_provider: Option<SignatureHelpOptions>,
    pub declaration_provider: Option<bool>,
    pub definition_provider: Option<bool>,
    pub type_definition_provider: Option<bool>,
    pub implementation_provider: Option<bool>,
    pub references_provider: Option<bool>,
    pub document_highlight_provider: Option<bool>,
    pub document_symbol_provider: Option<bool>,
    pub code_action_provider: Option<bool>,
    pub code_lens_provider: Option<CodeLensOptions>,
    pub document_link_provider: Option<DocumentLinkOptions>,
    pub color_provider: Option<bool>,
    pub document_formatting_provider: Option<bool>,
    pub document_range_formatting_provider: Option<bool>,
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    pub rename_provider: Option<bool>,
    pub folding_range_provider: Option<bool>,
    pub execute_command_provider: Option<ExecuteCommandOptions>,
    pub selection_range_provider: Option<bool>,
    pub linked_editing_range_provider: Option<bool>,
    pub call_hierarchy_provider: Option<bool>,
    pub semantic_tokens_provider: Option<SemanticTokensOptions>,
    pub moniker_provider: Option<bool>,
    pub type_hierarchy_provider: Option<bool>,
    pub inline_value_provider: Option<bool>,
    pub inlay_hint_provider: Option<bool>,
    pub diagnostic_provider: Option<DiagnosticOptions>,
    pub workspace_symbol_provider: Option<bool>,
    pub workspace: Option<ServerCapabilitiesWorkspace>,
    pub experimental: Option<LSPAny>,
}

pub struct InitializedParams {
    // TODO
}

pub struct SetTraceParams {
    pub value: TraceValue,
}

pub struct LogTraceParams {
    pub message: String,
    pub verbose: Option<String>,
}

pub struct DidOpenTextDocumentParams {
    pub text_document: TextDocumentItem,
}

pub struct TextDocumentContentChangeEvent {
    // TODO
}

pub struct DidChangeTextDocumentParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

pub struct DidSaveTextDocumentParams {
    pub text_document: TextDocumentIdentifier,
    pub text: Option<String>,
}

pub struct DidCloseTextDocumentParams {
    pub text_document: TextDocumentIdentifier,
}

pub struct CallHierarchyClientCapabilities {
    pub dynamic_registration: Option<bool>,
}

pub struct DocumentLinkParams {
    pub text_document: TextDocumentIdentifier,
}

pub struct CodeLensParams {
    pub text_document: TextDocumentIdentifier,
}

pub struct SemanticTokensClientCapabilitiesRequests {
    pub range: bool,
    pub full: bool,
}

pub struct TokenFormat {
    // TODO
}

pub struct SemanticTokensClientCapabilities {
    pub dynamic_registration: Option<bool>,
    pub requests: SemanticTokensClientCapabilitiesRequests,
    pub token_types: Vec<String>,
    pub token_modifiers: Vec<String>,
    pub formats: Vec<TokenFormat>,
    pub overlapping_token_support: Option<bool>,
    pub multiline_token_support: Option<bool>,
    pub server_cancel_support: Option<bool>,
    pub augments_syntax_tokens: Option<bool>,
}

pub struct MonikerClientCapabilities {
    pub dynamic_registration: Option<bool>,
}

pub struct Diagnostic {
    // TODO
}

pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub version: Option<i32>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct DocumentColorParams {
    pub text_document: TextDocumentIdentifier,
}

pub struct Color {
    // TODO
}

pub struct ColorInformation {
    pub range: Range,
    pub color: Color,
}

pub struct ColorPresentationParams {
    pub text_document: TextDocumentIdentifier,
    pub color: Color,
    pub range: Range,
}

pub struct ColorPresentation {
    pub label: String,
    pub text_edit: Option<TextEdit>,
    pub additional_text_edits: Option<Vec<TextEdit>>,
}

pub struct FormattingOptions {
    // TODO
}

pub struct DocumentFormattingParams {
    pub text_document: TextDocumentIdentifier,
    pub options: FormattingOptions,
}

pub struct DocumentRangeFormattingParams {
    pub text_document: TextDocumentIdentifier,
    pub range: Range,
    pub options: FormattingOptions,
}

pub struct DocumentOnTypeFormattingParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub ch: String,
    pub options: FormattingOptions,
}

pub struct RenameParams {
    pub new_name: String,
}

pub struct SymbolKind {
    // TODO
}

pub struct WorkspaceSymbolClientCapabilitiesSymbolKind {
    pub value_set: Vec<SymbolKind>,
}

pub struct SymbolTag {
    // TODO
}

pub struct WorkspaceSymbolClientCapabilitiesTagSupport {
    pub value_set: Vec<SymbolTag>,
}

pub struct WorkspaceSymbolClientCapabilitiesResolveSupport {
    pub properties: Vec<String>,
}

pub struct WorkspaceSymbolClientCapabilities {
    pub dynamic_registration: Option<bool>,
    pub symbol_kind: Option<WorkspaceSymbolClientCapabilitiesSymbolKind>,
    pub tag_support: Option<WorkspaceSymbolClientCapabilitiesTagSupport>,
    pub resolve_support: Option<WorkspaceSymbolClientCapabilitiesResolveSupport>,
}

pub struct DidChangeConfigurationParams {
    pub settings: LSPAny,
}

pub struct FileEvent {
    // TODO
}

pub struct DidChangeWatchedFilesParams {
    pub changes: Vec<FileEvent>,
}

pub struct MessageType {
    // TODO
}

pub struct ShowMessageParams {
    pub r#type: MessageType,
    pub message: String,
}

pub struct ShowMessageRequestParams {
    pub r#type: MessageType,
    pub message: String,
    pub actions: Option<Vec<MessageActionItem>>,
}

pub struct MessageActionItem {
    pub title: String,
}

pub struct LogMessageParams {
    pub r#type: MessageType,
    pub message: String,
}

