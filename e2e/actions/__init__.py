"""Action 层 — 可复用的业务操作片段。"""

from .app import launch_app, quit_app, restart_app, wait_app_ready
from .toolbar import click_refresh, open_commit_dialog, open_settings, switch_to_log_tab, switch_to_changes_tab
from .branch import open_branch_popup, search_branch, switch_branch, close_branch_popup
from .commit import type_commit_message, confirm_commit, cancel_commit
from .stash import stash_changes, pop_stash
from .navigation import open_remotes, open_tags, open_stashes, open_rebase, back_to_changes, close_panel
from .diff import select_file, show_diff, next_hunk, prev_hunk, next_file, prev_file, toggle_diff_mode
from .staging import stage_all, unstage_all, stage_selected, unstage_selected, toggle_view_mode
