# -*- coding: utf-8 -*-
"""
Утилиты и вспомогательные функции
"""

import re
from typing import List, Optional


def normalize_column_names(columns: List[str]) -> List[str]:
    """
    Нормализует имена колонок (lowercase, strip)
    """
    normalized = []
    for name in columns:
        if name is None:
            normalized.append("")
            continue
        normalized.append(str(name).strip().lower())
    return normalized


def find_column(possible_names: List[str], columns: List[str]) -> Optional[str]:
    """
    Ищет колонку по списку возможных имен
    """
    for candidate in possible_names:
        if candidate in columns:
            return candidate
    for candidate in possible_names:
        for col in columns:
            if col.startswith(candidate):
                return col
    return None


def has_any(text: str, keywords: List[str]) -> bool:
    """
    Проверяет наличие хотя бы одного ключевого слова в тексте
    """
    if not isinstance(text, str):
        return False
    lower = text.lower()
    return any(k in lower for k in keywords)
