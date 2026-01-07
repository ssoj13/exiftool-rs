# Bug Hunt

## Prereqs

- Answer in Russian in chat, write English code and .md files.
- MANDATORY: Use filesystem MCP to work with files, memory MCP to remember, log things and create relations and github MCP or "gh" tool if needed.
- Use sub-agents and work in parallel.

## Workflow

- Это Rust-порт C++ пакетов для VFX pipelines. Референсный код в _ref/. Задача этого порта - портировать C++ на Rust одновременно улучшая архитектуру, но по возможности сохраняя совместимость. Удобнее, быстрее, модульнее, расширяемее.
- Нужно проверить соответствие этого порта с оригиналами в _ref, реализованные и нереализованные разделы и фичи. Нужно внимательно прошерстить всё и отыскать все несовпадения. Нужно составить подробные списки всего что нужно улучшить и дописать, нам нужна полная Parity с оригиналами, или быть лучше их.
- Мы можем использовать другую логику или API если это приведёт к улучшениям и не изменит качество результата. Parity report - Это очень важно.
- Нужно тщательно проверить все преобразования, логику, абсолютно каждое, не пропуская ничего! Do not assume, check everything, step by step!
- Сделай для себя подробный план в TODO.md и строго следуй ему, обновляя файл по мере прогресса.

- это частичный порт _ref/exiftool на чистый Rust. Вроде всё работает.
- Изучи возможность полного порта всего exiftool, во всех деталях. Читать и писать любые форматы, полный порт, с Makernotes, со всем инструментарием (составить список).
- Изучи возможность создания интерфейса, может быть трейта для exiftool-rs для лёгкого подключения и отключения новых форматов файлов - сразу с makernotes и прочей требухой. Можно так сделать?
- Изучи подробно _ref/exiftool и наш exiftools-rs и составь карту паритета: что сделано, что нет, и что нужно сделать для полного порта. Возможно вносить архитектурные улучшения, т.к. оригинальный exiftool писался очень давно и вообще на перле.


- Check the app, try to spot some illogical places, errors, mistakes, unused and dead code and such things.
- Check interface compatibility, all FIXME, TODO, all unfinished code - try to understand what to do with it, offer suggestions.
- Find unused code and try to figure out why it was created. I think you haven't finished the big refactoring and lost pieces by the way.
- Check possibilities for code deduplication and single source of ground truth about entities, items and logic in app.
- Unify the architecture, dataflows, codepaths, deduplicate everything, simplify but keeping the logic and functionality! Do not remove features!
- Avoid of creation of special function with long stupid names in favor of arguments: just add the optional argument to existing function if needed.
- Do not guess, you have to be sure and produce production-grade decisions and problem solutions. Consult context7 MCP use fetch MCP to search internet.
- Create a comprehensive dataflow for human and for yourself to help you understand the logic.
- Do not try to simplify things or take shortcuts or remove functionality, we need just the best practices: fast, compact and elegant, powerful code.
- If you feel task is complex - ask questions, then just split it into sub-tasks, create a plan and follow it updating that plan on each step (setting checkboxes on what's done).
- Don't be lazy and do not assume things, do not guess code. You need to be SURE, since you're writing a production code. Do not simplify things unless it will significantly improve the code logic.
- Discard any compatibility issues, we don't need it.
- Create comprehensive report so you could "survive" after context compactification, re-read it and continue without losing details. Offer pro-grade solutions.

## Outputs

- At the end create a professional comprehensive detailed report in QWN.md with nice looking dataflow and codepath diagrams.
