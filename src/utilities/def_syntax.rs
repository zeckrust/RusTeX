// Defintions for LaTeX syntax

// Document
pub const DEF_BEGIN_DOCUMENT: &str = r"\begin{document}";
pub const DEF_END_DOCUMENT: &str = r"\end{document}";
pub const DEF_DOCUMENT_CLASS: &str = r"\documentclass";

// Package
pub const DEF_PACKAGE: &str = r"\usepackage";
pub const USER_PACKAGES_COMMENT: &str = r"% Added packages";
pub const DEFAULT_PACKAGES_COMMENT: &str = r"% Default packages";
pub const DEFAULT_FLOAT_PACKAGE: &str = r"\usepackage{float}";
pub const DEFAULT_GRAPHICX_PACKAGE: &str = r"\usepackage{graphicx}";

// Command
pub const USER_COMMANDS_COMMENT: &str = r"% Added commands";

// Enumerate
pub const DEF_BEGIN_ENUMERATE: &str = r"\begin{enumerate}";
pub const DEF_END_ENUMERATE: &str = r"\end{enumerate}";
pub const DEF_ITEM_ENUMERATE: &str = r"\item";

// Figure
pub const DEF_BEGIN_FIGURE: &str = r"\begin{figure}";
pub const DEF_END_FIGURE: &str = r"\end{figure}";
pub const DEF_INCLUDE_GRAPH: &str = r"\includegraphics";

// Table
pub const DEF_BEGIN_TABULAR: &str = r"\begin{tabular}";
pub const DEF_END_TABULAR: &str = r"\end{tabular}";
pub const DEF_BEGIN_TABLE: &str = r"\begin{table}";
pub const DEF_END_TABLE: &str = r"\end{table}";
pub const DEF_HORIZONTAL_LINE: &str = r"\hline";

// Section
pub const DEF_SECTION: &str = r"\section";
pub const DEF_SUB_SECTION: &str = r"\subsection";
pub const DEF_SUB_SUB_SECTION: &str = r"\subsubsection";

// Chapter
pub const DEF_CHAPTER: &str = r"\chapter";

// Page Break
pub const DEF_NEW_PAGE: &str = r"\newpage";

// Caption
pub const DEF_CAPTION: &str = r"\caption";

// Positioning
pub const DEF_CENTERING: &str = r"\centering";

// Formatting
pub const DEF_BOLD: &str = r"\textbf";
pub const DEF_ITALIC: &str = r"\textit";