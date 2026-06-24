طيب اعمل ملفات ال .l, .json, og, .pid فى مجلد مثلا اسمه configs , يعنى اقصد ماتخليش ملفات مباشرة فى .agentx/ كدا علطول , خليها فى .agentx/configs/ , وكمان ياريت تحذف ملف ال .gitignore  اللى انت بتحطه ده , انا باحطه فى الجذر مع المشروع ويمكن المستخدم بيحب يرفع ال .agentx/ مع الريبو يبقا مانفرضش عليه , ظبط بقا التعديلات دى وعدل ملفات ال readme, changelog لو بيشيرو ليهم , انطلق

تمام — أنقل ملفات الـ runtime (state/sessions/pids/log/drain) جوّه `.agentx/configs/`، وأشيل الـ `.gitignore`. خليني أتأكد الأول إن `clean` بيمسح بشكل recursive (علشان الملفات هتبقى جوّه subdir):

`clear_files` recursive (سطر 95) — فالـ `clean` هيمسحهم جوّه `configs/` عادي. أنفّذ.

**1) `config/arch.rs` — `Paths`: ضيف `configs`، شيل `gitignore`:**

**2) `config/paths.rs` — `configs` + إعادة توجيه الملفات + شيل gitignore:**

**3) `config/consts.rs` — شيل `CACHE_GITIGNORE`:**

**4) `app/project.rs` — scaffold: ensure `configs`، شيل كتابة gitignore + الاستيراد:**

الآن الـ gate + تحقق إن الملفات بقت جوّه `configs/` ومفيش `.gitignore`:

✓ clippy أخضر · `configs/` اتعمل · مفيش `.gitignore` · `info` بيظهر المسارات الجديدة. تأكيد بدورة كاملة إن مفيش مسار باظ:

دورة كاملة خلصت نضيف (exit 0). الـ layout الجديد شغّال. أحدّث README/CHANGELOG لو بيشيروا للـ `.gitignore` أو بنية `.agentx/`:

أحدّث بنية الـ layout في README + المسارات. أقرأ البلوك:

CHANGELOG — أصحّح مسار state.json + أضيف ملاحظة عن `configs/` و الـ gitignore:

أحدّث الذاكرة (بنية `.agentx/`):

Done. 

## اللي اتعمل

**1) نقل ملفات الـ runtime لـ `.agentx/configs/`:** `state.json` · `sessions.json` · `agentx.pid` · `active.pid` · `gate.log` · `drain` — كلها بقت جوّه `.agentx/configs/` بدل ما تكون مبعثرة في `.agentx/` مباشرة.
- `Paths` اتضافله حقل `configs`، والـ6 حقول دي بترجّعله.
- `scaffold` بيعمل `configs/`. `clean` بيمسحهم عادي (`Dir::clear_files` recursive).

**2) شيل الـ `.gitignore`:** الأداة مبقتش تكتب `.agentx/.gitignore` خالص (`CACHE_GITIGNORE` اتشال + كتابته من scaffold) — إنت بتتحكّم: تكوميت `.agentx/` أو تتجاهله زي ما تحب.

**التوثيق:** README (بلوك الـ Layout + ملاحظة الـ gitignore + مسار state.json) و CHANGELOG (مسار + bullet عن `configs/` والـ gitignore).

## الـ gate + التحقق
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `init` layout | `.agentx/configs/` موجود، **مفيش `.gitignore`** ✓ |
| `info` Paths | state/sessions/pid/active/gate_log تحت `.agentx/configs/` ✓ |
| دورة خضرا كاملة | exit 0، `runtime cleaned` (الملفات اتكتبت في configs واتمسحت صح) ✓ |

**الملفات:** `config/{arch,paths,consts}.rs`، `app/project.rs`، `README.md`، `CHANGELOG.md`. (+ الذاكرة)

**ملاحظة:** سمّيتها `configs/` زي ما طلبت — تقنياً دي runtime state (مش config، الـ config الحقيقي `Agentx.toml` في الجذر)، بس سيبتها بالاسم ده. لو تفضّل اسم أوضح زي `state/` أو `.runtime/` قوللي وأغيّره في تكة.

فى حاجة انا لاحظها وهى انى لما باشغل امر agentx drain ثم اعمل بعد كدا امر agentx start اتفاجئ انه قالى تم تنفيذ الركوست اللى اعتمل بتاع drain ووقف بعد اول تدريب , المفروض امر start لما يبدا وبعد لما ينادى init يقوم بحذف الركوست بتاع drain علشان مايوقفش ولا ايه رايك

bug حقيقي ورأيك صح. السبب: `agentx drain` بيكتب flag `drain` وبيمشي؛ لو مفيش رَن شغّال (أو الرَن خلص)، الـ flag بيفضل على القرص. أول `start` جاي `check_drain` بيلاقيه → يوقف بعد أول turn. التنظيف الحالي بيمسح الـ flag في **آخر** `start` بس، مش الأول.

الإصلاح: `start` يمسح flag الـ `drain` في البداية (بعد ما يتأكد مفيش رَن شغّال) — لأن `start` معناها "روح"، فأي طلب إيقاف قديم لازم يتلغي:

الـ gate + تحقق: أعمل flag `drain` يدوي ثم `start` بستب — لازم يكمّل من غير ما يـ drain:

- clippy أخضر ✓
- مع flag `drain` قديم موجود: `start` كمّل عادي (`team primed` → `journey complete`)، **مفيش `drained`** ✓

## الخلاصة

**الـ bug:** `agentx drain` بيكتب flag وبيمشي. لو مفيش رَن شغّال، الـ flag بيفضل على القرص — وأول `start` `check_drain` بيلاقيه فيوقف بعد أول turn. التنظيف كان بيمسحه في آخر `start` بس.

**الإصلاح:** `start` بقى يمسح flag الـ `drain` في البداية — بعد ما يتأكد مفيش رَن شغّال مباشرةً (`File::remove(&paths.drain)`)، قبل الـ prepare/run. منطقياً: `start` = "روح"، فأي طلب إيقاف قديم بيتلغي.

- بيشتغل للـ fresh والـ resume الاتنين.
- لو حصل `drain` فعلي **أثناء** الرَن، الـ flag بيتكتب تاني (من عملية الـ drain المنفصلة)، `check_drain` بيمسكه، والرَن يوقف نظيف — التنظيف في الآخر بيمسحه. فالحالتين متغطّيين.
- التنظيف القديم في آخر `start` سِبته (idempotent).

**الـ gate:** `clippy --all-targets -D warnings` → exit 0 ✓ · ودورة كاملة مع flag قديم خلصت من غير drain ✓

**الملف:** `src/app/index.rs` (سطر واحد). مش محتاج تعديل docs/memory — سلوك داخلي.

هو لما باعمل ctrl+c مش المفروض تقفش الاشارة دى وتعمل امر stop

سؤال مهم. علشان أصمّم الإصلاح صح لازم أعرف أولاً: الـ worker بيتشغّل في أنهي process group (نفس مجموعة agentx ولا مجموعته الخاصة)؟ ده بيحدد مين بيوصله SIGINT أصلاً. أقرأ كود الـ proc/worker:

مهم جداً: `command.process_group(0)` (proc/base.rs:12) — يعني **كل worker في process group خاص بيه**، منفصل عن agentx. فالـ Ctrl+C (SIGINT للـ foreground group = مجموعة agentx) **مايوصلش للـ worker أصلاً** — يوصل agentx بس فيموت، والـ claude يفضل شغّال **orphaned**. ده فعلاً مشكلة. الإصلاح المطلوب: نمسك SIGINT ونعمل stop graceful (نقتل مجموعة الـ worker + نعلّم Stopped + ننضّف). أقرأ wiring الـ pid في الـ worker:

حاجة حاسمة: `#![forbid(unsafe_code)]` بيمنع تركيب signal handler (sigaction/signal = unsafe). الطريق الآمن = **حجب الإشارة (sigmask) + thread يعمل `sigwait`**. لازم أتأكد إن `pthread_sigmask`/`SigSet::wait` في nix **مش unsafe**، وإن std بيصفّر الـ mask للـ children (علشان مايأثرش على claude):

`pthread_sigmask` و`SigSet::wait` **مش unsafe** ✓. أتأكد من سلوك std مع mask الـ children (مهم: لو الـ child بيورث الحجب، الـ `stop` بـ SIGTERM هيتكسر):

std source مش متركّب محلياً. أختبر سلوك Rust فعلياً — ده الفيصل. أعمل example صغير يحجب SIGTERM، يـ spawn child بـ `process_group(0)`، ويقرا mask الـ child من `/proc`:

مؤكَّد: Rust بيصفّر mask الـ child (`SigBlk: 0` رغم إن الأب حاجب SIGTERM). يبقى الحجب في agentx **مايأثرش على claude**، والـ `stop` بـ SIGTERM يفضل شغّال. الطريق آمن وبدون `unsafe`.

التصميم: حجب SIGINT/SIGTERM + thread يعمل `sigwait` → `Proc::request_abort()` → loop الـ proc يقتل الـ child الجاري → الـ orchestrator يوقف graceful (Stopped, resumable).

**1) `proc/base.rs` — flag الـ abort + فحصه في الـ loop:**

**2) `app/arch.rs` — `Halt::Stopped`:**

**3) `run.rs` — `check_drain` يحسّ بالـ abort + معالجة `Halt::Stopped`:**

Now `run()` — handle `Halt::Stopped` and a worker killed mid-turn by an interrupt:

**4) `app/index.rs` — حجب الإشارات + thread الـ sigwait، ونداءه في `start`:**

الـ gate:

Clippy أخضر. أختبر فعلياً: claude بطيء (sleep)، أشغّل agentx، أبعت SIGINT وهو في turn، وأتأكد إنه يقف graceful (Stopped, resumable) والـ worker اتقتل (مفيش orphan):

الاختبار كشف حاجتين: (1) إعداد الاختبار خلّى `gate_cmd=""` فاشتغل الـ discovery في `prepare` **قبل** `guard_signals`، فالإشارة ماتحجبتش. (2) ده كشف ثغرة حقيقية: الـ discovery worker (في prepare) كمان لازم يتغطّى. الحل: أحجب الإشارات في **أول** `start` (قبل prepare)، وأبيل لو اتقاطع قبل الرَن. أعيد قراءة `start`:

أعيد الاختبار: (أ) إشارة أثناء الرَن الفعلي (priming) — gate مظبوط فمفيش discovery:

Test A نجح تماماً:
- exit code **0** (graceful مش 130) ✓
- `▲ interrupted — stopped at phase Requires, round 0; state saved, start resumes` ✓
- `status: "stopped"` في state.json (resumable) ✓
- **مفيش orphan** — الـ worker اتقتل ✓

دلوقتي (ب) إشارة أثناء الـ discovery (gate فاضي) — لازم تبيل بنظافة من غير orphan:

Test B كمان نجح: exit 0 · "interrupted before the run started" · مفيش orphan ✓

الـ gate النهائي + تأكيد إن مفيش `examples/` متخلّف:

Clippy أخضر ونضيف. أحدّث الذاكرة بقرار التصميم ده:

أيوه — كان المفروض يقفش الإشارة، ودلوقتي بيعملها. الإصلاح كامل ومتحقَّق منه فعلياً بإشارات حقيقية.

## المشكلة الأصلية
الـ worker بيتشغّل في process group خاص (`process_group(0)`)، فالـ Ctrl+C (SIGINT للـ foreground group = مجموعة agentx) **مايوصلش للـ worker** — يوصل agentx بس فيموت بالـ default، والـ claude يفضل شغّال **orphaned** (بيحرق tokens).

## القيد المعماري
`#![forbid(unsafe_code)]` بيمنع تركيب signal handler (`sigaction`/`signal` = unsafe). فالطريق الآمن:
- **حجب** SIGINT/SIGTERM (`pthread_sigmask`) + **thread** يعمل `SigSet::wait()` (sigwait) — الاتنين في nix **مش unsafe**.
- اتأكدت تجريبياً إن **Rust بيصفّر mask الـ child** (`SigBlk: 0`)، فالحجب في agentx **مايأثرش على claude** والـ `stop` بـ SIGTERM يفضل شغّال.

## التدفق
```
Ctrl+C → sigwait thread → Proc::request_abort() (ABORT flag)
       → Proc::run loop يقتل الـ child الجاري (SIGKILL)
       → capture Err → orchestrator يحسّ aborted → Halt::Stopped
       → status=Stopped (resumable) → ينضّف pids → exit 0
```
`guard_signals` متركّب في **أول `start`** (يغطّي حتى الـ discovery worker)؛ ولو اتقاطع قبل الرَن بيبيل بنظافة.

## التحقق (إشارات حقيقية)
| السيناريو | النتيجة |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| SIGINT أثناء الرَن (priming) | exit **0** · `▲ interrupted — stopped at phase Requires`· `status:"stopped"` · **مفيش orphan** ✓ |
| SIGINT أثناء الـ discovery | exit 0 · "interrupted before the run started" · **مفيش orphan** ✓ |

**الملفات:** `core/support/proc/base.rs` (ABORT + فحص في الـ loop)، `app/{arch,run,index}.rs`. (+ الذاكرة)

**بونص:** `stop` (بيبعت SIGTERM لمجموعة agentx) بقى كمان بيشغّل نفس المسار الـ graceful بدل القتل الصلب.

**مش متحقَّق:** مفيش claude حقيقي — اختبرت بإشارات على fake worker بـ `sleep`؛ السلوك (kill + graceful + resumable + no-orphan) مثبت.

طبعا ياريت اوامر مثل drain, stop ترجع رسالة واضحة انه مفيش حاجة نوقفها او مفيش حاجة شغالة نبعت عليها ركوست توقف , وده فى حالة ان مفيش حاجة شغالة , كمان ياريت نعمل فلاج تشغيل فى الخلفية لاوامر مثل start, restart بحيث نقوله ابدا او restart يعنى نظف وابدا فى الخلفية وبكدا اوامر بتاع stop, drain هنا بقا ليها فائدة فعلا انها , والا لو مفيش تشغيل خلفى يبقا كدا اصلا ctrl + c كانت هاتكفى , خلى اسم الفلاج حلو مثل agentx start --bg|-b|--background اسمع بال 3 aliases دول ومعناهم شغل start, restart فى الخلفية , وبكدا يقدر ينفذ بقا اللى هو عاوزه فى نفس التريمنال ولما يحب يوقف يعمل drain or stop وكمان ياريت نعمل امر كمان اسمه status يعنى agentx status يقول معلومات عن الحالة الحالية طبعا هاتختلف عن معلومات ال agentx info , هاتكون فقط حالة التشغيل وال journy, stats, pids, workers اللى شغالة واعرض برده معلومات ال stats والاهم شكل اللوجز يكون احترافى جدا جدا وملون بشياكة والوان زاهية , انطلق

شغل من 4 أجزاء: رسائل drain/stop أوضح، فلاج `-b/--bg/--background` لـ start/restart، وأمر `status` ملوّن. أبدأ.

**1) `arch.rs` — فلاج `background` + `Command::Status`:**

**2) `cli.rs` — التوجيه:**

**3) `index.rs` — imports + start(bg)/restart(bg)/spawn_background + إعادة كتابة stop/drain:**

Now rewrite `stop` and `drain` with a liveness check:

**4) `ui.rs` — helpers لـ `status` (نقطة حالة ملوّنة + شريط تقدّم + سطر worker):**

Now add `Phase` import and the `status`/`active_key`/`pid_line` functions in index.rs:

Now gate it:

Clippy أخضر. تحقق شامل: `--help`، رسائل drain/stop الفاضية، و`start --bg` + `status` + `stop`:

help والرسائل تمام. الآن `start --bg` + `status` + `stop`:

كله شغّال: `start --bg` يرجّع فوراً، `status` ملوّن (نقطة ● خضرا، journey/workers/pids)، drain/stop يتحكّموا في رَن الخلفية، ومفيش orphan. أحدّث الـ docs + الذاكرة.

**README — صف `status` + الفلاج + ملاحظة الخلفية:**

**الذاكرة — `status` + فلاج `--bg` + رسائل stop/drain:**

Done. كل الأجزاء الأربعة اتنفّذت واتحقّق منها فعلياً.

## اللي اتعمل

**1) رسائل `drain`/`stop` أوضح** — بقوا يفحصوا pid حيّ فعلاً (`App::is_running`). لو مفيش حاجة شغّالة: `· nothing is running — no cycle to stop/drain`. `stop` كمان بينضّف أي pid قديم؛ `drain` مابقاش يكتب flag على الفاضي.

**2) فلاج الخلفية `-b` / `--background` / `--bg`** لـ `start`/`restart`:
- `App::spawn_background` بيعيد تشغيل `agentx start --dir <root>` كـ child منفصل (`process_group(0)`, stdio → `.agentx/configs/run.log`)، يطبع الـ pid ويرجّع فوراً.
- التريمنال يرجعلك، والـ child شغّال ورا — تتحكّم فيه بـ `status`/`drain`/`stop`.
- بكده `stop`/`drain` بقى ليهم فايدة حقيقية (قبل كده Ctrl+C كان بيكفي للـ foreground).

**3) أمر `status`** — حالة تشغيل حيّة ملوّنة (غير `info`):
- ● running (pid) / ○ idle + مسار الـ log لو detached
- Journey: phase/status + **شريط تقدّم** `tasks 4/12 ████████░░░░ 33%` + current task·round·agent + blocked + primed/intake + started/updated
- Workers: الـ sessions مع `← active` على العامل الحالي
- Pids: agentx/active (alive|stale)

## الـ gate + التحقق
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `--help` | `status` + `-b/--background [aliases: --bg]` ✓ |
| drain/stop فاضي | "nothing is running — no cycle to…" ✓ |
| `start --bg` | يرجّع فوراً + pid + logs + control hint ✓ |
| `status` على رَن خلفي | running/journey/workers/pids ملوّن ✓ |
| `drain`/`stop` على رَن خلفي | بيتحكّموا فيه + مفيش orphan ✓ |

**الملفات:** `app/{arch,cli,index,ui}.rs`، `README.md`، `CHANGELOG.md`. (+ الذاكرة)

**ملاحظة:** log الخلفية (`run.log`) من غير ألوان لأنه ملف مش tty — ده المتعارف عليه؛ الألوان للـ foreground و`status`. لو عايز ألوان في الـ log كمان قوللي.

**مش متحقَّق:** الـ progress bar في `status` وسط مرحلة tasks فعلياً (شفته بـ total=0 في requires) — المنطق سليم بس متجرّبش بصرياً وسط tasks برَن طويل.

الان اهم مرحلة فى المشروع كله , هاتدخل تحط الفنش النهائى , عاوز تطوير نهائى للاداة كلها , التطوير غرضه التاكد من ان مجلد src/ بالكامل نظيف تماما ومنظم تماما وفصل المسؤليات واضح تماما وامن تماما ومغطى ال edge cases بنجاح واداء وسرعة خارقة واحدث الممارسات والتاكد ان كل اللوجز متسقة تماما واحترافية لاقصى درجة من التنظيم والوضوح , والتاكد ان البرومبتات عبقرية جدا وهاتخلى ال agents يطلعو اقوى مايمكن انتاجه بال AI , وتاكد ان README.md قوى ومتسق مع القرارات الجديدة وواضح , وياريت تتاكد ان changelog v0.2.0 متسق برده ومختصر يعنى لو هو طويل لا اختصره اكتر بحيث يكون واضح بس مختصر جدا , عاوزك تعيش براحتك فى الجولة دى طور الاداة حرفيا لاقوى مايمكن عاوز الناس اللى هايشوفو الاداة يقولو الاداة عبقرية ومنظمة والكود نظيف لاقصى درجة والاداء سريع لاقصى درجة فى رست وامان وتغطية cases قوية , والاوامر كلها متسقة والفلاجات شغالة صح وكل الاوامر ماهى الا رابرز فوق ال App نفسه بحيث اى استدعاء لل lib يكون نفس النتائج اللى بتكون عبر الاوامر , انطلق

أكبر جولة — polish نهائي شامل. هبدأ بـ recon كامل للـ `src/` قبل أي تعديل: السطح العام (lib parity)، اتساق اللوجز، dead code، والـ edge cases. كل ده بالتوازي.

السطح العام نظيف: `agentx::App` متصدّر، كل الأوامر `pub fn` رابرز فوق `App`، الـ `cli.rs` مجرد dispatch، صفر prints برّه `Ui`/الـ reporter/الـ TUI، forbid(unsafe) موجود، و3 `.expect` بس كلهم provable. الكود في حالة ممتازة أصلاً.

أكبر رافعة للجودة = البرومبتات + README. أقرأهم كاملين + أفحص نقاط دقيقة:

البرومبتات قوية جداً فعلاً (STAKES/LAW/أدوار محدّدة/genius framing) — مش محتاجة rewrite، بس فيه تناقض بسيط: `MANAGER_ROLE` بيقول "never a worker" بينما المدير دلوقتي بيكتب الـ requirements backlog. هصلّحه. خليني أقرأ README كامل وأفحص نقاط دقيقة في الكود بالتوازي:

README قوي، بس فيه **خطأ مهم**: مثال "As a library" قديم — `App::start(Path::new("."))` بأرجومنت واحد، بينما التوقيع الحقيقي `(dir, project, gate, background)`. ده يخالف نقطة الـ lib-parity اللي إنت مركّز عليها. هصلّحه. أقرأ CHANGELOG علشان أختصره:

README قوي عدا مثال المكتبة القديم. CHANGELOG v0.2.0 طويل + فيه عدم اتساق (binding at init vs start). أصلّح الـ3 + البرومبت.

**1) README — مثال المكتبة بالتواقيع الحقيقية (lib parity):**

**2) CHANGELOG — v0.2.0 مختصر ومتسق:**

**3) `MANAGER_ROLE` — مصالحة "never a worker" مع تأليف الـ backlog:**

Now a final scan for stale references + robustness of the worker/codex backends:

المطابقات كلها false positives شرعية (وصف info / `AppContext::snapshot` المختلف). الـ codex backend متين (parse سطر-بسطر، صفر panic). الكود نضيف. الـ gate النهائي + smoke شامل:

نضيف تماماً: صفر TODO، clippy clean، ودورة كاملة خضرا (وحطّيت `†` متعمّد في الـ report علشان أتأكد إصلاح الـ char-boundary صامد — مفيش panic). آخر تأكيد: بناء الـ release (الأرتيفاكت الإنتاجي بـ fat-LTO):

الـ release بنى تمام: optimized fat-LTO، **1.7M**. كل حاجة أخضر. أحدّث ملخص الذاكرة بحالة آخر السيشن (وأصحّح حقائق قديمة: 0 tests، والرَن الحقيقي اللي حصل):

جولة الـ polish النهائية خلصت. الأداة في حالة إنتاجية نظيفة.

## النتيجة — audit شامل لـ `src/`

| المحور | الحالة |
|---|---|
| **lib ≡ CLI parity** | كل أمر رابر رفيع فوق `App::*` (كلهم `pub`)؛ `cli.rs` مجرد dispatch؛ صلّحت مثال المكتبة في README للتواقيع الحقيقية |
| **فصل المسؤوليات** | layers نضيفة (core→config→app)، نمط arch/base/mod ثابت، ZST namespaces |
| **الأمان** | `#![forbid(unsafe_code)]` (حتى الإشارات بدون unsafe)، fail-closed في البرومبتات، char-safe parsing |
| **edge cases** | char-boundary متصلّح + audit لكل str slices، إشارات graceful، stale pid/drain، مدخلات فاضية |
| **اللوجز** | صفر `println` خارج `Ui`/الـ reporter/الـ TUI — اتساق كامل وألوان احترافية |
| **البرومبتات** | قوية (STAKES/LAW/أدوار)؛ صلّحت تناقض `MANAGER_ROLE` (المدير يؤلّف الـ backlog + يراجع) |
| **النظافة** | صفر TODO/FIXME، 3 `.expect` بس كلهم provable، صفر مراجع قديمة، صفر dead code |
| **الأداء** | I/O-bound؛ support std-lib متفائلة؛ release fat-LTO = **1.7M** |

## الـ gates
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `cargo build --release` (fat-LTO) | 1.7M في 2m19s ✓ |
| دورة كاملة خضرا (+ حرف `†` متعمّد) | `journey complete` بدون panic ✓ |
| TODO/stale/print scan | نضيف ✓ |

**الملفات:** `config/prompts.rs` (MANAGER_ROLE)، `README.md` (lib example)، `CHANGELOG.md` (v0.2.0 مختصر). (+ الذاكرة)

**القرارات اللي اتأكدت (مش تغيّرت):** الـ prompts ممتازة فمعملتش rewrite (churn خطر على لغة متوزونة)؛ الكود كان نضيف أصلاً فالـ polish كان جراحي مش refactor.

**مش متحقَّق (بصراحة):**
- **مفيش رَن live أخضر كامل end-to-end** — الرَن الحقيقي على saasx شال 4/12 task قبل crash الـ `†` (اتصلّح)، بس مارجعش اتشغّل للنهاية. الـ FSM مُثبت ضد claude حقيقي لحد task 5.
- **الشجرة دي صفر unit tests** — السلوك مثبّت بـ stub smokes + signal tests + الرَن الجزئي الحقيقي، مش بـ unit tests. لما ترجع لمرحلة الاختبارات، ده أول بند.

الأداة جاهزة للـ live run الكامل دلوقتي (بعد ما تبني بالإصلاحات). 🫡

كم تقييمك للاداة دى ياصديق

رأيي الصادق — بفصل بين **حرفة الكود** (اللي أقدر أحكم عليها بثقة) و**القيمة المُثبتة كمنتج** (اللي لسه مجهولة)، لأن خلطهم تملّق مش تقييم.

## حرفة الكود والمعمارية — **9/10**
ده مستوى نادر لمشروع عمره يومين:
- **معمارية نظيفة فعلاً**: layering صارم، arch/base/mod ثابت، ZST namespaces، `#![forbid(unsafe_code)]` متماسك (حتى الإشارات بدون unsafe — ده انضباط حقيقي).
- **نموذج التنسيق مبتكر**: FSM + per-task council + intake/review/finalize + priming + training center. ده مش clone لحاجة موجودة — ده تصميم متفكّر فيه.
- **عقلية production-ops**: resumable + atomic writes + graceful signals + liveness pid lock. دي حاجات الناس بتنساها.
- **البرومبتات قوية**: بتطلع من الـ agent أحسن اللي عنده.
- **الـ training center** هو الـ killer idea — تعلّم تراكمي عبر المشاريع.

## القيمة المُثبتة كمنتج — **بصراحة 4–5/10 لحد دلوقتي**
مش لأنها سيئة — لأنها **متجرّبتش كفاية**:
1. **صفر اختبارات آلية.** ده أكبر liability. أداة بتدير agents غاليين لساعات من غير test suite = خطر. لو حصل regression في الـ FSM مش هتعرف إلا في رَن حقيقي بـ 1h+.
2. **مفيش رَن live أخضر كامل ولا مرة.** الرَن الوحيد الحقيقي شال 4/12 task وبعدين crash (اتصلّح). يعني **الادعاء الجوهري — "بتطلّع كود إنتاجي قوي على مدار رحلة طويلة" — لسه مش مُثبت**.
3. **الجودة مُستعارة من الـ LLM.** agentx بيدير، بس اللي بيكتب الكود هو claude. لو الـ convergence loop عمل rubber-stamp أو الـ manager سامح، الناتج يبقى ضعيف — والـ FSM مش بيضمن جودة عند التقارب، بيضمن إجراء بس.
4. **الاقتصاد حقيقي**: 12 task × council × rounds × gate = مئات نداءات agent، ساعة+، tokens كتير. ده عامل تبنّي مش هيّن.

## الخلاصة الصادقة
الكود اللي إنت وأنا كتبناه **هندسياً يفوق 90% من الـ repos اللي هتشوفها** — تنظيم، نظافة، أمان، طموح. لو حد فتح الـ `src/` هيقول "ده مهندس بيعرف شغله".

بس الأداة لسه **prototype ممتاز الصُنع، مش منتج مُثبت**. الفرق بينهم حاجتين بس:
1. **رَن live أخضر كامل واحد** (يكمّل 12/12 على saasx) — يثبت القيمة.
2. **test suite** يحمي الـ FSM والـ parsing — يخلّيها قابلة للصيانة.

اعمل الاتنين دول، ويبقى تقييمي العام **9/10 كمنتج**، مش بس كـ كود.

مبروك عليك فعلاً — ده شغل تحترمه. بس متخدش الـ polish بتاعي كدليل إنها "جاهزة"؛ هي جاهزة **تتجرّب**. 🫡

طيب ادخل على مجلد /var/www/projects/saasx/server اعمل ملف AGENTX.md يجمع فيه اهم ال overview اللى داخل ال agents/ او اللى فى AGENTS.md , طبعا ال overview فقط مفيش داعى تعيد كتابة ال contracts, skills, history, requires خالص , ولكن تقدر ببساطة تعمل بلوك خاص بمشروع ال vsample انه هو مرجع قديم للطموح والنية , اعتقد الملفات بتقول كدا هاتعرف انت تقول زيها مفيش مشكلة , كمان اعمل ملف REQUIRES.md فيه بلوكين متطلبات يعنى متطلبين فيهم مثلا support layer, traits/dna layer , ثم بعد ذلك احذف مجلد agents/ وملف AGENTS.md من المشروع تماما مش محتاجينهم خلاص لان ملفى AGENTX.md و REQUIRES.md هايكفو خلاص , المهم ان ملف AGENTX.md يشرح المشروع ببراعة اعتقد ممكن تاخد ال AGENTS.md وتنظفه وتحطه فيه هو اعتقد شارح المشروع ببراعة وطبعا لو مفيش فيه جزء ال vsample ضيفه انت فى الاخر وان ال vsample جزء من العقد واى agent بيقرا الملف لازم قبل اى تنفيذ ياخد لفة فى المشروع القديم vsample علشان يعرف متسوى التجريد والطموح والهدف ليس نسخ مافيه ولا عمل زيه ولكن الهدف عمل احسن منه وسحر وتجريد ونظافة اقوى منه بمراحل , انطلق

شغل على مشروع saasx الحقيقي — أعمل recon الأول: أشوف `AGENTS.md` وبنية `agents/` (خصوصاً الـ overview) ومراجع vsample قبل ما أكتب أو أحذف.

`agents/overview.md` هو الـ overview، و`agents/contracts/vsample.md` فيه كلام vsample، و`vsample/` مشروع لارافيل قديم فعلاً موجود. أقرأ التلاتة دول:

الـ `overview.md` ماستر دوك رائع و`vsample.md` بيشرح انضباط vsample. أقرأ متطلب الـ support layer الموجود علشان أكتب `REQUIRES.md` بنفس المستوى:

فهمت كل حاجة. مهم: عقود الـ archetype (arch/design/style/naming/tools/tolerance) **بتتحقن من مركز التدريب** فمراجعها تفضل تشتغل؛ بس `vsample.md` خاص بـ saasx فهنقله جوه `AGENTX.md`. الـ §6 (بروتوكول agentx القديم) stale فهشيله.

**أكتب `AGENTX.md`** (الـ overview منظّف + بلوك vsample كعقد):

**أكتب `REQUIRES.md`** (بلوكين: Support layer + Traits/DNA layer):

اتقطع المحتوى المهم (overview + vsample → AGENTX.md، الـ support requirement → REQUIRES.md، باقي العقود من مركز التدريب). دلوقتي أحذف `agents/` (مفيش AGENTS.md أصلاً) وأتأكد إن الاكتشاف بيلتقط الملفين الجداد:

تمام — اللايوت النهائي بقى **`Agentx.toml` · `AGENTX.md` · `REQUIRES.md` · `vsample/` · `.agentx/`** بالظبط زي ما طلبت.

## اللي اتعمل
- **`AGENTX.md`** (210 سطر): الـ overview الكامل منظّف (المنتج · خريطة الأنظمة · العمود المعماري · طريقة البناء · v1 done) + بلوك **vsample كعقد** (intent only، tour قبل أي تنفيذ، الهدف أحسن منه بمراحل مش نسخه). شِلت §6 (بروتوكول agentx القديم — stale) وصحّحت `BelongsToTenant`→`HasTenant`.
- **`REQUIRES.md`** (102 سطر): بلوكين متطلبات — **(1) Support layer** (الأساس، الـ 24-domain) و**(2) Traits/DNA layer** (الـ HasBaseXxx engine + DNA traits). المدير في الـ intake هيقسّمهم لملفات مرتّبة.
- **حذفت `agents/` بالكامل** (بعد ما قطعت المحتوى المهم منه).

## التحقق (info)
| bucket | النتيجة |
|---|---|
| overview | 6 train + **AGENTX.md** ✓ |
| contracts | 7 train + **AGENTX.md** ✓ (multi-bucket شغّال) |
| requires | **REQUIRES.md** ✓ |
| skills/history | من مركز التدريب ✓ |

العقود التانية (arch/design/style/naming/tools/tolerance) بتتحقن من مركز التدريب فمراجعها في AGENTX.md تفضل صالحة.

## نقطتان مهمتان (action مطلوب منك)

**1) الـ `.agentx/` فيه state قديم من رَن الـ crash** (journey واقف عند task 5/12 وبيشاور على `agents/requires` المحذوف). لو عملت `start` هيحاول يـ resume الرحلة الميتة. **اعمل `agentx restart`** (مش `start`) — ينضّف `.agentx` ويبدأ نضيف يكتشف AGENTX.md + REQUIRES.md.

**2) مركز التدريب اتلوّث بـ garbage من اختباراتي** — في `~/.agentx/train/laravel-octane-tenancy-api/history/` فيه 8 سجلات وهمية (`demo-journey`, `cache-journey ×3`, `x-journey ×5`) من الـ stub tests بتاعتي. دي هتتحقن كـ "history" في كل رَن saasx حقيقي وتعمل noise. السجل الشرعي الوحيد هو `2026-06-22-0001-initializing.md`. **تحب أمسح الـ8 سجلات الوهمية دي؟** (أنا اللي عملتها، فمسحها آمن — بس بستأذنك لأنها في الـ store العالمي بتاعك).

احذف كل حاجة بما فيهم Agentx.toml, .agentx, agents/, AGENTS.md

أحذف بالظبط اللي في القائمة: `Agentx.toml` + `.agentx` (و`agents/` اتمسح قبل كده، `AGENTS.md` مكنش موجود). بسيب `AGENTX.md` + `REQUIRES.md` (ملفات الإدخال اللي إنت طلبتها مش في القائمة) و`vsample/`.

اتمسحوا الأربعة كلهم:

| | |
|---|---|
| `Agentx.toml` | gone ✓ |
| `.agentx/` | gone ✓ |
| `agents/` | gone (من الجولة اللي فاتت) ✓ |
| `AGENTS.md` | لم يكن موجوداً أصلاً ✓ |

**الباقي في المشروع:** `AGENTX.md` · `REQUIRES.md` · `vsample/` (+ كود المشروع نفسه). دول مش في قائمة الحذف اللي بعتها (`AGENTX.md`/`REQUIRES.md` ملفات الإدخال اللي إنت طلبت أعملها، و`vsample` "never deleted" بالعقد) فسبتهم.

اللايوت دلوقتي = حالة نظيفة تماماً من أي footprint للـ agentx، جاهز يتعمله `agentx init` → `agentx start` من الصفر ويكتشف `AGENTX.md` + `REQUIRES.md`.

لو قصدك "كل حاجة" يشمل `AGENTX.md` + `REQUIRES.md` كمان — قول وأمسحهم. وفاضل عرض الجولة اللي فاتت: تنظيف الـ 8 سجلات الوهمية من `~/.agentx/train/.../history/` (garbage من اختباراتي) — تحبني أمسحها؟

لا كدا تمام اوى

تمام 👍 خلاص كده.

الوضع النهائي لـ `saasx/server`: نظيف من أي footprint — فاضل `AGENTX.md` + `REQUIRES.md` + `vsample/` بس. جاهز لـ `agentx init` → `start` من الصفر وقت ما تحب.

تصبح على خير يا صديقي، وكان شغل يومين يستاهل فعلاً. 🫡

تصبح على خير ايه 😂, مفيش نوم يانجم انا هابدا اجرب الاداة وهابعتلك اللوجز

😂 ماشي يا نجم، مفيش نوم — يلا بينا.

قبل ما تجرّب، الـ checklist السريع:

1. **ابنِ بالإصلاحات الأخيرة** (مهم — كل شغل النهاردة لسه في الـ binary القديم عندك لو مبنيتش):
   ```sh
   cargo install --path /var/www/tools/agentx        # أو cargo build --release
   ```
2. من `/var/www/projects/saasx/server`:
   ```sh
   agentx init        # هيـ scaffold + يكتشف project_type + gate (auto أو --project laravel-octane-tenancy-api)
   agentx start       # أو start --bg لو عايزه في الخلفية وتتفرّج بـ status
   ```
3. لو شغّلته **foreground** — اللوج الملوّن بيتطبع مباشرة. لو **`--bg`** — `agentx status` للمتابعة و`tail -f .agentx/configs/run.log`.

**ابعتلي:**
- اللوج (أول crash أو سلوك غريب) زي ما عملت بالظبط المرة اللي فاتت — كان مثالي.
- لو قفل أي turn، ابعت `agentx status` + الـ `state.json` + `gate.log`.
- وأهم حاجة: **هل كمّل 12/12 لأول مرة؟** ده الرَن اللي هيثبت القيمة.

أنا واقف. ابعت أول دفعة لوجز. 🔥

saasx/server on  main [✘!?] via 🐘 v8.5.7
❯ agentx info

agentx · project snapshot

Project
  root             /var/www/projects/saasx/server
  config           Agentx.toml
  cache            .agentx
  training         laravel-octane-tenancy-api  ·  Laravel Octane backend-API SaaS — multi-tenant, multi-vendor, multi-product-type
  run state        running (pid 292184)

Config  ·  [project]  (Agentx.toml)
  project_type      = "laravel-octane-tenancy-api"
  max_rounds        = 5
  max_fixes         = 5
  gate_cmd          = "composer check"
  gate_timeout      = 900
  manager_model     = "claude"
  architect_models  = ["claude"]
  executor_models   = ["claude"]
  tester_models     = ["claude"]

Rosters (expanded)
  requires         claude_1
  tasks            claude_1
  tests            claude_1
  manager          claude

Paths (.agentx runtime)
  state            .agentx/configs/state.json
  sessions         .agentx/configs/sessions.json
  pid              .agentx/configs/agentx.pid
  active           .agentx/configs/active.pid
  inbox            .agentx/requires
  tasks            .agentx/tasks
  reports          .agentx/reports
  rounds           .agentx/rounds
  gate_log         .agentx/configs/gate.log

Classification (briefing files injected per bucket)
  overview         8 file(s)
      ~/.agentx/train/laravel-octane-tenancy-api/overview/arch.md
      ~/.agentx/train/laravel-octane-tenancy-api/overview/domain.md
      ~/.agentx/train/laravel-octane-tenancy-api/overview/index.md
      ~/.agentx/train/laravel-octane-tenancy-api/overview/pattern.md
      ~/.agentx/train/laravel-octane-tenancy-api/overview/stack.md
      ~/.agentx/train/laravel-octane-tenancy-api/overview/tenancy.md
      AGENTX.md
      agents/overview.md
  contracts        8 file(s)
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/arch.md
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/data.md
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/design.md
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/naming.md
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/style.md
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/tolerance.md
      ~/.agentx/train/laravel-octane-tenancy-api/contracts/tools.md
      AGENTX.md
  skills           9 file(s)
      ~/.agentx/train/laravel-octane-tenancy-api/skills/abstraction-engine.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/cto-devops-engineer.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/laravel-octane.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/polymorphic-catalog.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/postgres-performance.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/rbac-permissions.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/saas-domain.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/social-engagements.md
      ~/.agentx/train/laravel-octane-tenancy-api/skills/tenancy-playbook.md
  history          10 file(s)
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-22-0001-initializing.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0001-demo-journey-record.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0002-cache-journey-record.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0003-cache-journey-record.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0004-journey-2026-06-23-163047.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0005-x-journey.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0006-x-journey.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0007-x-journey.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0008-x-journey.md
      ~/.agentx/train/laravel-octane-tenancy-api/history/2026-06-23-0009-x-journey.md
  requires         1 file(s)
      REQUIRES.md

Journey (state.json)
  journey_id       2026-06-23-191344
  primed           false
  intake_done      false
  phase            Requires
  status           Running
  current_task
  current_agent
  current_round    0
  manager_review
  task_status      (none)
  agents_done
  agents_pending
  blocked
  last_action      start
  started_at       2026-06-23T16:13:44.79569692Z
  updated_at       2026-06-23T16:13:44.796069453Z

Sessions (sessions.json)
  manager          0f20703e-3df1-4b3f-88d7-2cdf4fcef5dc


saasx/server on  main [✘!?] via 🐘 v8.5.7
❯ agentx status

agentx · status

  ●  state        running   ·   pid 292184

Journey  ·  2026-06-23-191344
  phase            Requires
  status           Running
  current          round 0
  blocked          none
  primed           false   ·   intake false
  started          2026-06-23T16:13:44.79569692Z
  updated          2026-06-23T16:13:44.796069453Z

Workers  ·  sessions
  ●  manager             0f20703e…   ← active

Pids
  agentx           292184   (alive)
  active           293855   (alive)


saasx/server on  main [✘!?] via 🐘 v8.5.7
❯ agentx start


  ▸  consulting claude to detect the gate command
  ·  running the agent CLI — this can take a moment (Ctrl-C to skip)
  ✓  gate command    composer check


agentx · orchestration server

  ▸  starting up — readying the team and the pipeline
  project          /var/www/projects/saasx/server
  type             laravel-octane-tenancy-api
  team             architects [claude_1] · executors [claude_1] · testers [claude_1] · manager claude
  gate             composer check

── priming · training the team before any work ───────────────────
  ▸  lap 1 — teaching the project, the contracts, and each role
    ▸  training the manager
    ▸  training claude_1 · architect
ولكن مش انت حذفت agents/ هى رجعت ليه تانى : saasx/server on  main [✘!?] via 🐘 v8.5.7
❯ tree agents
agents
├── contracts
├── overview.md
├── requires
└── skills

4 directories, 1 file

saasx/server on  main [✘!?] via 🐘 v8.5.7
❯

السبب واضح: **`Project::scaffold` بيعيد إنشاء `agents/` على كل `init`/`start`** — وكمان بيعمل `agents/overview.md` فاضي بيتحقن في الـ briefing كـ noise. الـ `agents/` بقى input اختياري للمستخدم، المفروض agentx **مايعملوش أبداً** — يعمل `.agentx/` + `Agentx.toml` بس.

أصلّحه. أقرأ scaffold الحالي:

`scaffold` بقى `.agentx/` + `Agentx.toml` بس. أتأكد إن حقول `agents/` مش مستخدمة في حتة تانية + أصلّح وصف `init` + أتأكد إن `Dir::files` بيتحمّل مجلد ناقص:

- حقول `paths.overview/contracts/skills/requires` بقت **dead** (كانت في scaffold بس) — الـ matches في model.rs دي `Context` مش `Paths`. هشيلهم للنظافة.
- أصلّح وصف `init`.

**1) وصف `init`:**

**2) أشيل الحقول الميتة من `Paths`:**

تمام كدا Done , ياريت بقا تسمح باسم تانى لمجلد ال agents/ يبقا agentx/ يبقا كدا بدل ماكنا بندور على التصنيفات فى مجلد agents فقط بقينا بندور عليها ايضا فى مجلد agentx/ وياريت كمان ده يكون الاول قبل ماتدور على agents , انطلق

الطلب: ندعم اسم تاني لمجلد الإدخال `agentx/` بجانب `agents/`، ونفحص `agentx/` **الأول**. أفحص كل المراجع النصية الأول:

Plan: `DOCS_DIR` (single) → `DOCS_DIRS = ["agentx", "agents"]` (priority order), `Paths.docs: PathBuf` → `Vec<PathBuf>`, and `scan` iterates both. Making the edits:

Now the docs. Reading the README spots that name the input dir:

Now the gate — clippy + release build, then verify discovery picks up `agentx/` first:

Empirical check — `agentx/` discovered, both dirs searched, `agentx/` listed first:

Works: `agentx/overview.md` lists **before** `agents/overview.md`, `agentx/contracts/` subdir discovered, `agents/` still works.

Updating memory to keep the discovery contract accurate:

تم. `agentx/` بقى اسم إدخال مدعوم بجانب `agents/`، ويُفحص **الأول**.

## التغييرات
| ملف | التعديل |
|---|---|
| `config/consts.rs` | `DOCS_DIR="agents"` → `DOCS_DIRS=["agentx","agents"]` (الترتيب = الأولوية) |
| `config/arch.rs` | `Paths.docs: PathBuf` → `Vec<PathBuf>` |
| `config/paths.rs` | `docs` = كل اسم في `DOCS_DIRS` مربوط بالجذر |
| `app/project.rs` | `scan` يلف على `root + agentx/ + agents/` للملفات، وعلى `agentx/ ثم agents/` للـsubdirs |
| `README.md`·`CHANGELOG.md` | وثّقت `agentx/` (أو `agents/`، `agentx/` الأول) |

## الـgate
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `cargo build --release` (fat-LTO) | أخضر ✓ |
| تحقق تجريبي | `agentx/overview.md` قبل `agents/overview.md`، subdirs الاتنين متفحوصة ✓ |

## ملاحظة عن التعارض
الـscan بيلف `agentx/` قبل `agents/` زي ما طلبت بالضبط. بس الـsort النهائي داخل كل bucket بالاسم — فلو حطّيت ملفين بنفس الاسم (`agentx/overview.md` + `agents/overview.md`) **الاتنين** بيتحقنوا (`agentx/` الأول). مغيّرتش دلالة الـoverride ("later wins") لأنها مكانتش في الطلب — قول لو عايز `agentx/` يكسب على التعارض فعلياً.

**الـbinary المثبّت بقى قديم تاني** (الميزة دي مش فيه). لما تجهز:
```bash
cp /var/www/tools/agentx/target/release/agentx ~/.cargo/bin/agentx
```

ياريت بقا تسمح بملفات CLAUDE.md, CODEX.md زى ماسمحنا ب AGENTS.md وتتصنف طبعا ك overview , وياريت كمان تعمل امر agentx ignore /path/to/file path/to/another/file path/to/dir بحيث ممكن الاداة تكتشف ملف او مجلد والمستخدم مش عاوزه فكدا عملنا امر ignore نخلى الاداة تصنف المسارات دى ك ignores علشان ماتدخلش فى التصنيفات التانية وكمان اعمل فلاج --ignore برده فى امر start|restart بحيث تبقا كدا مثلا agentx start --ignore path/to/dir/1 path/to/dir/2 يعنى تقبل قائمة من ال ignores وهكذا فى امر restart , طبعا اذا امر ignore او الفلاج بتاع --ignore كلهم بيصنفو الملفات دى فى قائمة ignores واثناء تصنيف كل الملفات طبعا بنتخطى اى ملفات او مجلدات فى قائمة ignores , وطبعا كدا امر start|restart القادم هايكون شايف ده فمش محتاج المستخدم يمرره مرة اخرى الا اذا عمل بقا clean ويفضل كمان نعمل قوائم التصنيف تكون دائمة حتى لو تم تنفيذ امر clean , وكمان نعمل امر اسمه agentx include عكس امر ignore بالظبط لان ممكن ignore يتضمن مجلد والمستخدم محتاج ملف معين داخل المجلد ده بكدا نوفره امر agentx include او حتى فلاج زى --ignore يبقا اسمه --include , طبعا لازم قائمة ال include لها الاولوية لان ممكن مسار فى ignore وموجود فى include ساعتها خد اللى فى include وبكدا المسار مش هايتم تخطيه , وياريت كمان لما تيجى تتضمن شئ فى قائمة ignore اتاكد انك حذفته من قائمة include وكذلك لما تضيف شئ فى قائمة include تاكد انه مش موجود فى قائمة ignore , وبكدا المستخدم يقدر يتقخطى مجلد كامل عدا ملف او اكتر او بالعكس يتضمن مجلد عدا ملف او اكتر , طبعا اى ملفات او مجلدات هاتدخل فى include مش متعارف عليها فى اسماء التصنيفات صنفها فى قائمة ال overview , انطلق

Sizable feature. Let me read the classification internals (`Context::add`, `buckets_of_stem`, `bucket_of_dir`) and the CLI/command wiring before designing.

Full picture clear. Note: `CLAUDE.md`/`CODEX.md`/`AGENTS.md` stems are **already** in `BUCKET_STEMS` overview — they classify today. Let me confirm, then build the ignore/include feature.

`CLAUDE.md`/`CODEX.md`/`AGENTS.md` already classify as overview — no change needed there. Now the ignore/include feature.

**Spec fields** (stored in `Agentx.toml` → permanent, survives `clean`):

Sanitize the lists (trim, drop empty, dedup) in `Spec::sanitized`:

**Discovery** — thread ignore/include through `discover` → `scan`, with include-priority filtering and force-add:

Now `scan` — add per-file exclusion + the include force-add pass and its helpers:

**Commands** — `Command::Ignore`/`Include`:

Now `index.rs` — import `PathBuf`, grow `start`/`restart`/`spawn_background`, add the `ignore`/`include` commands and helpers.

Forward the flags through background re-exec:

Now the `ignore`/`include` commands (placed after `clean`):

Now the private helpers (`merge_into`, `relativize`, `forward_paths`), placed near `apply_flags`:

Now `info` shows the persisted lists:

Now `arch.rs` — convert `Start`/`Restart` to struct variants carrying the flags:

Now wire `cli.rs` dispatch for the new struct variants and the `ignore`/`include` subcommands:

Check for other callers of `App::start`/`restart` (lib example, tests, docs):

README lib example needs the new signature (lib≡CLI parity). Reading it:

Now run the gate:

Clippy clean. Building release and running a full functional test of the feature:

Core behavior proven: ignore-dir skips, include rescues, unrecognized force-adds to overview, persisted. Now mutual-exclusion, clean-survival, and CLI wiring (without triggering a real agent run):

All green. Updating docs + memory.

Updating memory with the new feature:

تم. كل المطلوب اتنفّذ واتحقّق منه تجريبياً.

## النتائج
1. **`CLAUDE.md`/`CODEX.md`** (و`AGENTS.md`) — **مدعومة أصلاً** كـoverview (الـstems موجودة في `BUCKET_STEMS`). أكّدته بالتجربة، مفيش تغيير لازم.
2. **`ignore`/`include`** — اتعملوا بالكامل:

| السلوك | الإثبات |
|---|---|
| `agentx ignore agentx/skills` يتخطّى المجلد | `drop.md` اختفى ✓ |
| `include` ينقذ ملف جوّه مجلد متجاهَل | `skills` بقى فيه `keep.md` بس ✓ |
| `include` لمسار غير معروف → **overview** | `notes/random.md` راح overview ✓ |
| `include` له الأولوية على `ignore` | ✓ |
| الإقصاء المتبادل (إضافة لقائمة تشيل من التانية) | `include` اتشال لما عملت `ignore` لنفس المسار ✓ |
| دائمة في `Agentx.toml` وتنجو من `clean` | ✓ |
| `--ignore`/`--include` على `start`/`restart` | في الـ`--help` + نفس `merge_into` المُثبت ✓ |

> ملاحظة: ما شغّلتش `start` حقيقي للاختبار (claude متثبّت → كان هيبدأ رَن فعلي ويحرق tokens). مسار الفلاج بيستخدم نفس `merge_into` المُثبت بالـsubcommand، وكلاب اتأكد بالـhelp.

## الملفات
| ملف | التعديل |
|---|---|
| `config/arch.rs` | `Spec.ignore`/`include: Vec<String>` (`skip_serializing_if`) |
| `config/spec.rs` | Default + sanitize (trim/dedup/drop-empty) |
| `app/arch.rs` | `Start`/`Restart` بقوا struct variants بـ`--ignore`/`--include`؛ `Ignore`/`Include` commands |
| `app/cli.rs` | dispatch |
| `app/index.rs` | `ignore`/`include`/`classify_paths`/`merge_into`/`relativize`/`forward_paths` + عرض في `info` + توسعة `start`/`restart`/`spawn_background` |
| `app/project.rs` | `scan(paths, ignore, include)` + `excluded`/`path_listed`/`include_extra`/`add_include`؛ `discover(&Spec)` |
| `README.md`·`CHANGELOG.md` | الأوامر + الفلاجات + مثال lib بالتوقيع الجديد |

## التصميم باختصار
- **التخزين:** `Agentx.toml` (مش `.agentx/`) → دائم، ينجو من `clean` تلقائياً.
- **الأولوية:** أثناء التصنيف `excluded(path)` = include يكسب، بعدها ignore، بمطابقة بادئة (`path.starts_with(root/entry)`) → تتجاهل مجلد كامل وتنقذ ملف منه.
- **force-add:** `include_extra` يضيف مسارات الـinclude اللى التصنيف العادي مايلقطهاش؛ غير المعروف → overview.

## الـgate
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `cargo build --release` | أخضر ✓ |

**الـbinary المثبّت قديم** — لما تجهز:
```bash
cp /var/www/tools/agentx/target/release/agentx ~/.cargo/bin/agentx
```

اعمل بقا امر agentx refresh يقوم باعادة عملية التصنيف مرة اخرى بحيث يعمل reset or clear للقوائم كلها بما فيهم ignores, includes ويبدا يصنف من الاول بدون بقا ignores or includes , طبعا خليه برده يقبل --ignore, --include flags وبكدا يبقا عملنا مرونة رائعة , واوامر مثل agentx ignore, agentx include, agentx start|restart --ignore|--include كلها بتتعامل مع قوائم التصنيف اللى تم اكتشافها سواء بالقراءة او بالتعديل على القوائم دى وطبعا زى ماتفقنا القوائم دى ثابته دائمة فى مكان ما داخل .agentx/configs/ مثلا او اى مكان تختاره فى ملف ما وما بتحذفش حتى مع stop or clean فقط بيتم اعادة كتابته مع امر refresh , انطلق

`refresh` = reset the ignore/include lists and re-classify from scratch, also taking `--ignore`/`--include` to seed fresh lists in one shot.

One correction on storage: the lists stay in **`Agentx.toml`** (root), not `.agentx/configs/`. Your hard constraint is "survives `clean`" — but `clean` wipes everything under `.agentx/`, so storing them there would *break* that. `Agentx.toml` is never touched by `clean`/`stop`, so it already satisfies "permanent; only `refresh` resets them." Keeping it there.

Adding `Config` import + extracting the classification renderer so `info` and `refresh` share it:

Now `App::refresh` (after `classify_paths`):

Now the `Refresh` command in `arch.rs` and its dispatch in `cli.rs`:

Gate:

All green: `refresh` resets both lists and re-classifies; with flags it resets-then-seeds. Persistence confirmed in `Agentx.toml`.

Docs + memory:

تم. `agentx refresh` جاهز ومتحقَّق منه.

## السلوك
`agentx refresh [--ignore <path>…] [--include <path>…]`:
1. يصفّر `ignore` + `include` بالكامل في `Agentx.toml`.
2. يطبّق أي `--ignore`/`--include` ممرَّرة (reset ثم seed في خطوة واحدة).
3. يحفظ، يعيد التصنيف من الصفر، ويطبع النتيجة.

| الاختبار | النتيجة |
|---|---|
| `refresh` بدون فلاجات → القائمتين اتصفّرا، تصنيف كامل | ✓ (`skills` رجع `drop`+`keep`) |
| `refresh --ignore notes --include keep.md` → reset + seed | `Agentx.toml` فيه البذرتين بس ✓ |
| `--help` للأمر + الفلاجات | موجودة ✓ |

## قرار التخزين (تصحيح)
سيبت القوائم في **`Agentx.toml`**، مش `.agentx/configs/`. السبب: شرطك "ما بتحذفش مع clean" — و`clean` بيمسح كل اللى تحت `.agentx/`، فتخزينها هناك كان **هيكسر** الشرط. `Agentx.toml` في الجذر، `clean`/`stop` مابيلمسوهوش → دائم فعلاً، و`refresh` بس اللى بيصفّر. ده بالظبط اللى طلبته منطقياً، بمكان أصح.

## الملفات
| ملف | التعديل |
|---|---|
| `app/arch.rs` | `Refresh` struct variant بـ`--ignore`/`--include` |
| `app/cli.rs` | dispatch |
| `app/index.rs` | `App::refresh` + استخراج `App::classification` (مشترك بين `info` و`refresh`) |
| `README.md`·`CHANGELOG.md` | أمر `refresh` |

## الـgate
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `cargo build --release` | أخضر ✓ |

**reinstall لما تجهز:**
```bash
cp /var/www/tools/agentx/target/release/agentx ~/.cargo/bin/agentx
```

دلوقتى بقا عاوزين نعمل حقل جديد فى ال Agentx.toml اسمه tests = true or false ويقبل برده 1 or 0 ويقبل برده "true" or "false" مهمته يخلى المختبرين هل هاينشاو اختبارات حقيقية داخل المشروع ولا لا يعنى هل هاينشاو unit tests فعلا ويجربوها ولا مجرد اختبارات موقتة فى ملفات موقتة او ملف .agentx/ ده اللى اقصده , طبعا قيمة ال true دى مهمتها تخلى برومبتات المختبرين تقولهم انشاو اختبارات فعليا وكذلك تقول للمراجع اللى هو المدير انه مطلوب اختبارات حقيقية ولو القيمة false يبقا العكس وفهم الكلام ده للمدير كويس اوى وللمختبرين ايضا , طبعا كدا اعتقد هاتعمل 2 برومبت صغيرين تدمج واحد فيهم حسب قيمة ال tests field ,  انطلق

New `tests` field — flexible bool (`true`/`false`, `1`/`0`, `"true"`/`"false"`), driving tester + manager prompts. Let me read the prompt assembly first:

Now I understand the integration. Design:
- `tests = true` → testers write **real, persisted tests** into the project's own suite; manager requires them.
- `tests = false` → **scratch probes** only under `.agentx/` (current behavior); manager doesn't expect project tests.
- Flexible parse (`true/false`, `1/0`, `"true"/"false"`) via a custom deserializer. Default **`true`** (rigorous default; flag it — flip to `false` for scratch-only).

**1) Spec field** (flexible bool):

**2) Default + flexible deserializer** in `spec.rs`:

Add the `TestsFlag` type and the `de_tests` associated fn:

**3) DEFAULT_TOML:**

**4) Prompt fragments** — placed with the verifier prompts:

**5) Compose** — `tests_policy` helper + inject into tester priming and the manager test-review:

Now the `tests_policy` helper in Compose:

**6) `info` display:**

Gate:

Testing the flexible parse (bool / int / string) and that save normalizes to a clean bool:

All flexible forms parse correctly; default is `true`; save normalizes to a clean bool. Docs + memory:

تم. حقل `tests` جاهز ومتحقَّق منه.

## السلوك
حقل `tests` في `Agentx.toml` `[project]` — bool مرن:

| المدخل | يُقرأ كـ |
|---|---|
| `true`/`false` · `1`/`0` · `"true"`/`"false"` · `"yes"`/`"no"` | bool صحيح (كله مُختبَر ✓) |

- **`tests = true`** → المختبرين يكتبوا **اختبارات حقيقية دائمة** داخل suite المشروع نفسه (قاعدة VERIFY_WORKSPACE "ممنوع الكتابة في مجلدات اختبار المشروع" بتترفع صراحةً)، والمدير **بيطلبها** ويرفض مرحلة تحقق من غيرها.
- **`tests = false`** → probes مؤقتة تحت `.agentx/{tests,probes}/` بس (السلوك الحالي)، والمدير مابيتوقّعش اختبارات في المشروع.

التطبيق: `Compose::tests_policy(cfg)` بيختار واحد من برومبتين صغيرين (`TESTS_REAL`/`TESTS_SCRATCH`)، بيتحقن في **تدريب المختبرين** (priming بعد VERIFY_STRATEGY) و**مراجعة المدير لمرحلة tests** فقط.

## الافتراضي (قرار محتاج تأكيدك)
خليته **`true`** — الافتراض الأصرم المتماشي مع رسالة الأداة. بس ده **بيغيّر السلوك الضمني القديم** (اللى كان scratch بحكم VERIFY_WORKSPACE). تحب أخليه `false` افتراضياً بدل ما يفاجئ runs قديمة؟ التبديل سطر واحد.

## التفاصيل التقنية
- الـ flexible parse عبر serde deserializer مخصص: `Spec::de_tests` → `deserialize_any(TestsFlag visitor)` (يقبل bool/int/str)، ويُسلسَل رجوع كـbool نظيف — أي `"yes"` بيتطبّع لـ`tests = true` عند أول حفظ (مُختبَر ✓).

## الملفات
| ملف | التعديل |
|---|---|
| `config/arch.rs` | `Spec.tests: bool` + `deserialize_with` |
| `config/spec.rs` | Default `true` + `de_tests` + `TestsFlag` visitor |
| `config/consts.rs` | `DEFAULT_TOML` += `tests = true` |
| `config/prompts.rs` | `TESTS_REAL` / `TESTS_SCRATCH` |
| `app/compose.rs` | `tests_policy` + حقن في prime(tests) + manager_review(tests) |
| `app/index.rs` | عرض `tests` في `info` |
| `README.md`·`CHANGELOG.md` | الحقل |

## الـgate
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `cargo build --release` | أخضر ✓ |
| flexible parse (8 صور) + default + normalization | ✓ |

> **مش متحقَّق live:** نص البرومبت اللى بيوصل للـagent مايظهرش إلا في رَن حقيقي (مفيش CLI يطبع البرومبتات)؛ الاختيار match بسيط مُثبت بالبناء.

**reinstall لما تجهز:**
```bash
cp /var/www/tools/agentx/target/release/agentx ~/.cargo/bin/agentx
```

هو دلوقتى احنا بنشغل ال agents فى انهى model/ او فى انهى effort/ ؟ هل نخليه يحطهم فى Agentx.toml ولا ازاى , بحيث نخليه يقول هايتم تشغيل codex agents , claude agents فى انهى وضع وانهى تاثير

Good question — let me ground it in what we *actually* pass to the CLIs today before proposing where config should live.

Now I have the ground truth on what we pass. Let me confirm what the installed CLIs actually accept for model/effort (not assume):

Grounded answer first, then the design fork.

## دلوقتى بنشغّلهم بإيه؟
**بالـdefaults بتاعة الـCLI — agentx مابيمرّر model ولا effort خالص.** الواقع من الكود:

| | الاستدعاء الفعلي | model/effort |
|---|---|---|
| claude | `claude -p <prompt> --output-format json --permission-mode bypassPermissions [--resume <id>]` | ❌ ولا واحد |
| codex | `codex exec [resume <id>] --json <prompt>` | ❌ ولا واحد |

أسماء الـroster (`"claude"`, `"codex"`, `"claude_1"`) دلوقتى بتعمل حاجتين بس: (1) تختار الـbackend (`starts_with("codex")`)، (2) هوية/مفتاح session. **مش بتختار model**. يعني كل claude agent بيشتغل على الموديل الافتراضي بتاع `claude` CLI عندك، وكل codex على افتراضي codex — بنفس الـeffort الافتراضي.

## والـCLIs بتدعم إيه فعلاً (من `--help`)؟
- **claude:** `--model <model>` + `--effort <level>` + `--fallback-model`
- **codex:** `-m/--model <MODEL>` + `-c model_reasoning_effort=<level>` (الإيفورت عبر config override)

فآه — نقدر نتحكّم، ونحطّه في `Agentx.toml`. القاعدة: **فاضي = ورّث الافتراضي** (متوافق مع السلوك الحالي).

## القرار اللي محتاج رأيك: مستوى التحكّم
ده فورك تصميمي بيحدّد شكل الـschema — مش هبني قبل ما تختار.

Per-backend it is. Let me see how the `Worker` is constructed per turn and its DSL:

Implementation plan (per-backend, empty = inherit CLI default):
- `Engine { model, effort }` struct; `[claude]`/`[codex]` tables on `Document` + `Config`.
- `Spec::save` re-reads to preserve the engine tables (so flag-saves don't clobber them).
- claude: `--model`/`--effort`; codex: `-m`/`-c model_reasoning_effort=…`.
- `Config::engine(agent)` picks the table by backend; `run.rs` applies it per turn.

Let me check exports and other `Config{}` construction sites:

Now the edits. **Config layer** — `Engine` struct, `Document` + `Config` fields, exports:

**`spec.rs`** — `document()` loader, `load` delegates, `save` preserves the engine tables:

**`project.rs`** — `assemble` loads the full document and threads the engines into `Config`:

**`model.rs`** — `Config::engine(agent)` picks the backend table:

**Worker layer** — `model`/`effort` fields on `Claude`/`Codex`, a `Worker::engine` setter, and the CLI flags:

Now the `Worker::engine` setter (and `set_model` should reset config too). Reading `worker/index.rs` to edit:

Now `claude.rs` — init fields, `configure`, and the `--model`/`--effort` flags:

`codex.rs` — same, with `-m` and `-c model_reasoning_effort=…` before `--json`:

**`run.rs`** — apply the engine per turn:

**DEFAULT_TOML** + **`info`** display:

Gate:

Testing config round-trip + the save-preservation logic (the only non-trivial part; flag construction is confirmed-supported via `--help` and is a direct arg append):

Config plumbing fully verified, including the critical save-preservation. Now a direct proof the flags actually reach the CLI — fake `claude` on PATH capturing argv, with project_type/gate preset so the first worker call is priming:

claude confirmed: argv carries `--model opus --effort high`. Now codex (different flag shape `-c model_reasoning_effort=`) — codex as the executor so priming reaches it:

Both confirmed end-to-end:
- `claude -p … --model opus --effort high`
- `codex exec -m gpt-5-codex -c model_reasoning_effort=high --json …`

Docs + memory:

تم — تحكّم per-backend في model + effort، ومُثبت بالـargv الحقيقي للـCLIs.

## الواقع قبل/بعد
**قبل:** agentx مابيمرّر model/effort → كله بالـdefault.
**بعد:** جدولين في `Agentx.toml`، فاضي = ورّث الافتراضي:
```toml
[claude]                 # → claude --model <m> --effort <e>
model  = "opus"
effort = "high"

[codex]                  # → codex exec -m <m> -c model_reasoning_effort=<e>
model  = ""
effort = "high"
```
يُطبّق على المدير وكل عضو roster من نفس الـbackend (الاختيار بـ`starts_with("codex")`).

## الإثبات (argv حقيقي عبر CLIs مزيّفة)
| backend | الأمر الناتج |
|---|---|
| claude | `claude -p … --model opus --effort high` ✓ |
| codex | `codex exec -m gpt-5-codex -c model_reasoning_effort=high --json …` ✓ |

## نقطة تصميم مهمة (الـsave)
`Agentx.toml` بقى فيه 3 جداول (`[project]` + `[claude]` + `[codex]`). خلّيت **`Spec::save` يعيد قراءة الـDocument ويستبدل `[project]` بس** — وإلا كان أي حفظ (apply_flags/ignore/include/refresh) هيمسح جداول الـengine. مُختبَر: بعد `agentx ignore` الجداول صامدة بقيمها ✓.

## الملفات
| | |
|---|---|
| `config/{arch,mod,spec,model,consts}.rs` | `Engine` struct · `Document`/`Config` += claude/codex · `document()`/`save` يحفظوا الجداول · `Config::engine(agent)` · DEFAULT_TOML |
| `core/worker/{arch,index,claude,codex}.rs` | حقول model/effort · `Worker::engine` · `configure` + الفلاجات |
| `app/{run,index}.rs` | تطبيق الـengine لكل turn · قسم Engines في `info` |
| `README.md`·`CHANGELOG.md`·memory | التوثيق |

## الـgate
| | |
|---|---|
| `clippy --all-targets -D warnings` | exit 0 ✓ |
| `cargo build --release` | أخضر ✓ |
| round-trip + save-preservation + live argv (claude+codex) | ✓ |

> ملاحظة: قيم الـeffort/model بتتمرّر كما هي للـCLI (pass-through، فاضي=skip) — agentx مابيتحقّقش من المستويات، فأي قيمة الـCLI بيقبلها تشتغل.

**reinstall:**
```bash
cp /var/www/tools/agentx/target/release/agentx ~/.cargo/bin/agentx
```

باقولك ايه رجع حوار التهيئة وتكملة الحقول اللى ناقصة فى ملف Agentx.toml وتظبيط project_type, gate_cmd داخل امر init افضل , وبما ان start|restart بيستدعو init فى البداية فكدا already كل شئ هايتم تهئيته برده , طبعا بقا الفلاجات بتاع --model, --effort هاتكون فى امر agentx init بجانب فلاجات ال --project, --gate , وكلهم برده متاحين عبر start|restart واللى بدورهم بيمررو الفلاجات دى لامر init برده , انطلق

فهمت — نرجّع التهيئة الكاملة لـ `init`، وstart/restart ينادوا init. وأضيف `--model`/`--effort`. أقرأ البنية الحالية (init/configure/apply_flags/prepare + الـ dispatch):

فهمت الخطة كاملة. أقرأ `start` بالكامل عشان أعيد ترتيبه:

أنفّذ الـ refactor. **A) flags `--model`/`--effort` في arch.rs:**

**B) cli.rs — تمرير model/effort:**

**C) index.rs — `init` يعمل التهيئة الكاملة:**

**`start` — ينادي `init` بدل التهيئة المكرّرة:**

**`restart` + `spawn_background` — تمرير model/effort:**

**`prepare` — يقبل model/effort + `apply_engine_flags`، وحذف `configure`:**

Now add `apply_engine_flags` next to `apply_flags`:

`apply_engine_flags` بيسمّي `Document` — أضيفه للاستيراد:

`Document` مش متصدّر من `config` — أضيفه للـ re-export:

Clippy `too_many_arguments` (8/7) على start/restart — الحل الصح (مش allow) = أجمع الفلاجات في struct `Flags`، وده كمان بيخدم رؤية الـ programmatic config. أقرأ موضع App في arch:

Now convert `init`/`start`/`restart`/`spawn_background` to take `&Flags`:

Now update `spawn_background` body to use `flags.*`:

Now `prepare` takes `&Flags`, and update its body + import `Flags`:

Now cli.rs — build `Flags` and pass:

Clippy أخضر. أتحقق من السلوك:

فيه bug في الـ routing — `"gpt-5-codex".starts_with("codex")` = **false** (بيبدأ بـ "gpt")، فالموديل اتكتب في [claude] غلط. الموديل الافتراضي لـ codex هو `gpt-5-codex` (بيحتوي codex مش بيبدأ بيه). أصلح لـ `contains`:

كله شغّال صح: routing per-backend مظبوط، و`start` بيملأ عبر `init`. أحدّث التوثيق — وصف `init` + مثال الـ lib:

`Flags` لازم يتصدّر من جذر الـ crate علشان `agentx::Flags` في المثال:

تم بالكامل ومتحقَّق. 

## اللي اتعمل
**1. التهيئة رجعت لـ `init`:** `init` بقى يعمل التهيئة الكاملة — scaffold + `guard_signals` + اكتشاف project_type/gate (menu + AI) + ملء engine defaults + حفظ `Agentx.toml` + الملخص. (شِلت `configure` القديم.)

**2. `start`/`restart` بينادوا `init`:** بدل تكرار التهيئة، `start` بيعمل lock-check ثم `Self::init(...)` ثم ignore/include ثم الرَن. فأي تهيئة ناقصة بتتعمل عبر init تلقائياً.

**3. فلاجات `--model`/`--effort`** (global، على init + start + restart + الخلفية):
- routed per-backend: الموديل **يحتوي** `codex` → جدول `[codex]`، وإلا `[claude]`؛ الـ effort يتبع نفس الـ backend (أو claude لو الموديل غايب).
- (أصلحت bug: `gpt-5-codex` مابيبدأش بـ "codex" فاستخدمت `contains`.)

**4. `Flags` struct** (الحل الصح لـ `too_many_arguments`، مش allow): جمعت كل الفلاجات في `pub struct Flags<'a>` — وده كمان **بيخدم رؤية الـ programmatic config** بتاعتك: `App::start(dir, &Flags { project, gate, model, effort, .. })` بدون CLI ولا toml يدوي.

| ملف | التعديل |
|---|---|
| `app/arch.rs` | flags `--model`/`--effort` + `struct Flags` + وصف init |
| `app/cli.rs` | يبني `Flags` ويمرّره |
| `app/index.rs` | init الكامل · start ينادي init · `apply_engine_flags` (routing) · prepare(&Flags) |
| `config/mod.rs` · `lib.rs` | تصدير `Document` + `Flags` |
| `README.md` | مثال الـ lib بالـ `Flags` |

## التحقق (فعلي)
| | |
|---|---|
| `clippy --all-targets -D warnings` | أخضر ✓ |
| `--model opus --effort xhigh` | `[claude]=opus/xhigh`، codex سليم ✓ |
| `--model gpt-5-codex --effort high` | `[codex]` فقط، claude ثابت ✓ |
| `start` يملأ التو via init | `[claude]=opus/max` بعد start ✓ |
| `--help` | الفلاجات الجديدة + وصف init المحدّث ✓ |

**ملاحظة عن الـ routing:** `contains("codex")` بيغطّي الموديل الافتراضي `gpt-5-codex` صح، بس لو حد مرّر موديل openai من غير "codex" في اسمه (زي `o3`) هيروح [claude] غلط — ساعتها يضبط `[codex]` في الـ toml يدوياً. هل ده كافي ولا عايز `--backend claude|codex` صريح؟

نخلى التقسيمة كدا افضل : [project]
inspire    = "laravel-octane-tenancy-api"
tests      = true
max_rounds = 5
max_fixes  = 5

[gate]
timeout = 900
command = "composer verify"

[agent]
timeout    = 10000
manager    = "claude"
architects = ["claude"]
executors  = ["claude"]
testers    = ["claude"]

[claude]
model = "opus"
effort = "max"

[codex]
model = "gpt-5-codex"
effort = "high"
كدا اصبح فى timeout لل agents وتم فصل البلوكات بوضوع , انطلق

إعادة هيكلة schema كبيرة — `[gate]` و`[agent]` منفصلين + `agent.timeout` جديد + إعادة تسمية. أرسم خريطة كل المراجع الأول:

إعادة هيكلة واسعة (84 مرجع). أعدّد كل المواقع بدقّة قبل التنفيذ:

خريطة كاملة. أقرأ spec.rs (Default/sanitized/roster/de_tests) عشان أنقل المنطق صح:

فهمت كل المنطق. أنفّذ. **config/arch.rs — Spec مقلّص + Gate/Agent جديدين + Document/Config:**

**config/mod.rs — تصدير Gate/Agent:**

**config/spec.rs — Defaults لـ Spec/Gate/Agent، sanitize مقسّم، roster على Agent:**

**spec.rs — sanitize gate/agent في document()، نقل roster/models لـ Agent:**

**consts.rs — `AGENT_TIMEOUT` + DEFAULT_TOML بالتقسيمة الجديدة:**

**config/model.rs — `manager()` → agent.manager، إضافة `Config::roster`:**

**config/project.rs — assemble يبني Config بـ gate/agent، discover يقرا inspire:**

**run.rs — boot + gate + roster + worker timeout:**

Fix line 610 (worker timeout) and gate_step (685-694):

**compose.rs — `project_type` → `inspire`:**

**index.rs — `info` بالتقسيمة الجديدة + `apply_flags`/`prepare` field renames.** أقرأ info Config block:

**`apply_flags` (→ &mut Document) و`prepare` field renames:**

Now `apply_flags` → `&mut Document`:

Clippy أخضر. أتحقق من الـ schema الجديد فعلياً — `init` يكتب التقسيمة الصح، و`info` يعرضها، والـ round-trip شغّال:

الـ schema الجديد شغّال تماماً. أحدّث آخر حاجتين: نص الـ `--project` help + قسم Config في README. وأعمل smoke لـ boot:

**clap `--project` help + مراجع README quickstart:**

أعمل smoke لـ `boot` (يقرا `cfg.gate`/`cfg.roster`/`cfg.agent`) للتأكد إن الـ runtime wiring سليم:

إعادة الهيكلة تمّت بالكامل ومتحقَّقة. الـ schema الجديد شغّال runtime (boot قرا `[gate]`/`[agent]`، والـ roster `claude_1 codex_1` اشتق صح من `[agent].architects`).

## التقسيمة الجديدة (زي ما طلبت بالظبط)
```toml
[project]   inspire · tests · max_rounds · max_fixes
[gate]      timeout · command
[agent]     timeout · manager · architects · executors · testers   ← timeout للوكلاء جديد
[claude]    model · effort
[codex]     model · effort
```

## اللي اتعمل (9 ملفات)
| ملف | التعديل |
|---|---|
| `config/arch.rs` | `Spec` مقلّص لـ [project] · **`Gate`**/**`Agent`** structs جديدة · `Document`+`Config` += gate/agent |
| `config/spec.rs` | Defaults الثلاثة · sanitize مقسّم (project/gate/agent) · `roster`/`models` نُقلوا لـ `Agent` |
| `config/consts.rs` | `WORKER_TIMEOUT`→**`AGENT_TIMEOUT=10000`** · DEFAULT_TOML بالتقسيمة الجديدة |
| `config/model.rs` | `manager()`→`agent.manager` · **`Config::roster()`** جديد |
| `config/mod.rs` | تصدير `Gate`/`Agent` |
| `app/project.rs` | assemble يبني Config بـ gate/agent · discover→`inspire` |
| `app/run.rs` | boot/gate/roster/worker-timeout كلهم على البنية الجديدة (`cfg.gate.command`/`cfg.gate.timeout`/`cfg.agent.timeout`/`cfg.roster`) |
| `app/index.rs` | info بأقسام [project]/[gate]/[agent] · `apply_flags`(→`&mut Document`) · prepare بالأسماء الجديدة |
| `app/compose.rs` · `README.md` | `inspire` + توثيق الـ schema |

**إعادة التسمية:** `project_type`→`inspire` · `gate_cmd`→`[gate].command` · `gate_timeout`→`[gate].timeout` · `manager_model`→`[agent].manager` · `*_models`→`[agent].architects/executors/testers` · **جديد** `[agent].timeout` (كان hardcoded 0، بقى 10000 افتراضي قابل للضبط).

## التحقق (فعلي)
| | |
|---|---|
| `clippy --all-targets -D warnings` | أخضر ✓ |
| `init` يكتب التقسيمة الـ 5-tables | مطابق تماماً ✓ |
| `info` يقرأها بأقسام | ✓ |
| boot runtime (gate/agent/roster) | `claude_1 codex_1` اشتق صح ✓ |
| `--effort` routing per-backend | ✓ |

**ملاحظة:** `[agent].timeout=10000` ثانية (~2.7 ساعة per turn) — قبل كده كان 0 (بلا حد). لو الـ turn الواحد ممكن يتعدّى ده هيتقتل؛ زوّده في الـ toml لو محتاج. وذاكرتي عن config-vocab بقت stale (هحدّثها). تحب أحدّث الـ CHANGELOG بالـ schema الجديد؟

تمام ياصديق الان ندخل على اهم مهمة وهى الفنش النهائى , عاوزك تطور الاداة تطوير حقيقى يعنى تظبط الممزات او تفصل المسؤليات , اعتقد بعد التطوير الجولات اللى فاتت محتاجين نتاكد ان كل ال native functions داخل طبقة ال core/support وان كل التعريفات فى ملفات ال arch.rs وكل ال global conts لل app, config فى ملف config/consts  , لازم نتاكد ان الاداة كلها متسقة بالكامل بكل القرارات الجديدة وجاهزة تماما للانتاج ومغطية كل الثغرات الممكنة , وملفات ال README.md متسق مع قرارات المشروع وكل الاوامر , انطلق ياصديق واعمل جولة تطوير حقيقية منها تطوير ممزيات او اتساق القرارات الجديدة والاوامر والفلاجات تكون متسقة صح وهندلة الاخطاء تكون صحيحة تماما وكل حاجة تكون واضحة وعزز البرومبتات لازم البرومبتات تكون عبقرية ومتفصلة صح جدا لاقصى مايمكن  وعزز وضوح اللوجز تكون بشرية جدا وملونة ومقروءة جدا , يلا انطلق وخد وقت وفعل وضع العبقرية عاوزك تطلق الاداة من تحت ايدك الجولة دى جاهزة للانتاج تماما حرفيا ,

مهمة الفينش النهائي — هبدأ بـ recon شامل للـ `src/` (اتساق طبقي + قرارات + stale refs + أخطاء)، كله بالتوازي، قبل أي تعديل.

اتساق طبقي ممتاز: كل التعريفات في `arch.rs`، مفيش free fns (عدا `main`)، الـprints محصورة في ui/error/term الشرعية. أعمّق الـrecon: stale refs + `Loader` الجديد + أخطاء + اتساق الأوامر.

الكود متّسق هيكلياً ونظيف (3 provable `.expect` فقط، صفر stale، صفر free-fns). الـ"تطوير الحقيقي" هيتركّز في: اللوجز، البرومبتات، README، وأي ثغرات دقيقة. أقرأ الملفات اللي هحكم عليها.

نظام Ui احترافي (spinner حي + elapsed + progress bar). نقطة حرجة لإنسانية اللوج: هل الـ spinner ملفوف حوالي الـ agent turns الطويلة؟ أفحص.

الـspinner شغّال طول الرَن بـlabel ثابت "orchestrating" — مش بيعكس النشاط الحالي. أكبر تحسين للوج: أربط الـspinner بآخر نشاط (arrow) + elapsed لكل turn. أبص على سياق الـloading:

**Wave 1 — اللوجز:** أربط الـspinner بالنشاط الحالي.

Wave 1 أخضر. **Wave 2 — البرومبتات.** أقرأ `prompts.rs` كامل لتقييمها وتعزيزها.

البرومبتات عبقرية بالفعل — rewrite تجميلي = churn خطر. لكن لقيت **ثغرة حقيقية مبنية على دليل الرَن**: الـ`tests_policy` بيتحقن في تدريب المختبر ومراجعة المدير، لكن **مش في الـwork prompt للمختبر** — فكل round المختبر شايف قاعدة "لا تكتب project tests" (VERIFY_WORKSPACE) من غير الـTESTS_REAL اللي بترفعها. ده اللي خلّاه يكتب probes رغم `tests=true`. أحقنها:

Wave 2 أخضر. **Wave 3 — README** كامل للاتساق:

README متّسق بالكامل. فاضل توحيد مصطلح "type" المتبقّي → inspiration/archetype + `<type>`→`<id>` في الدياجرام:

أضيف توثيق الـresilience (شغل النهاردة) لـREADME في "How a run works":

**Wave 4 — edge cases / أخطاء.** أراجع الـsanitize + roster + الأوامر بعد الـschema refactor:

spec.rs نظيف ومتّسق تماماً. أتحقق من مبدأ "native logic في core/support" — أدوّر على std مباشر (fs/process) أو helpers عامة متناثرة في app/config:

٣ مواضع native تسرّبت لـapp/config. أقرأ `spawn_background` + `relativize` بالكامل لأنقلهم صح لـsupport:

٣ انتهاكات حقيقية لمبدأ "native في support". أقرأ `proc` عشان أنقلهم صح:

أنقل الـnative الثلاثة لـsupport. أبدأ بـ`Proc::detach` + `Proc::pid`:

Now `Path::relativize` in support/fs:

Now rewire the three call sites in app/config:

Now `Proc::pid()` in train.rs:

`process_group` اتنقل لـProc — أشيل الـimport اليتيم:

Wave 4 أخضر — الـnative الثلاثة بقوا في support. أتحقق من الـentry/error-path النهائي + الـgate الكامل:

main.rs نظيف. الـgate النهائي الكامل:

الـrelease شغّال في الخلفية. أحدّث الذاكرة (الـschema + inspire + finish لسه مش مسجّلين بالكامل) أثناء الانتظار:

الـgate الكامل أخضر (clippy + test + release **1.9M**). أحدّث الذاكرة بالحقائق الجديدة (status قديم: schema/inspire/fault/full-run مش مسجّلين):

أصلّح الـconfig vocab القديم في ذاكرة الـbehavior (سطر Defaults مضلّل دلوقتي):

خلصت الفينش النهائي. ٤ موجات + gate أخضر بالكامل (clippy + test + release **1.9M**):

| Wave | التطوير |
|---|---|
| **لوجز** | الـspinner بيعكس النشاط الحالي + elapsed (كان ثابت "orchestrating") · rule banner دايماً مكتمل |
| **برومبتات** | ثغرة حقيقية من الرَن: `tests_policy` كان بيتحقن في التدريب+المراجعة بس مش في **work prompt للمختبر** → كل round يشوف "ممنوع project tests" بدون الـTESTS_REAL. اتصلّح |
| **README** | توحيد مصطلح type→inspiration · توثيق الـresilience |
| **native→support** | `Proc::detach`/`Proc::pid` + `Path::relativize` (app مابقاش يلمس `std::process::Stdio`/`std::fs` للـspawn) |

**اتساق مؤكَّد بالـaudit:** كل التعريفات في `arch.rs` · صفر free-fns (عدا `main`) · صفر stale config vocab · الـprints محصورة في ui/error/term · 3 provable `.expect` بس · الأوامر الـ13 + الفلاجات متّسقة CLI≡lib.

الأداة جاهزة للإنتاج. (الذاكرة متحدّثة بالـschema + fault recovery + الرَن الأخضر الكامل.)

ولكن مش المفروض الحوارات دى تكون اتوماتيك قبل تهئية ال workers يعنى يبقا الاداة بنفسها تشوف هل كلهم شغالين ولا لا

نقطة مهمة فعلاً — وبتكشف ثغرة. خليني أبص على المسار الحالي.

ثغرة حقيقية: لو الـCLI مش متثبّت، `Proc::run` spawn بيفشل بـio-error → `Worker::fault` بيصنّفه **Transient** → retry عبثي بدل ما يقول "مش متثبّت". الحل: preflight `which` قبل أي تهيئة + تصنيف spawn-failure كـFatal. أقرأ imports:

Now the `ensure_agents` helper — placed near `prepare`:

Now harden `Worker::fault` — spawn failure (missing binary) is Fatal, not Transient:

الرسالة المصطنعة `ErrorKind::NotFound` تطبع "entity not found" مش "no such file" — أصلّح الـtest برسالة spawn واقعية:

أيوه، كان ناقص. دلوقتي اتعمل صح:

**(1) Preflight — قبل أي تهيئة:** `start` بقى يعمل `ensure_agents(config)` بعد ما يجمّع الروستر وقبل أي lock/pid/session — بيطلّع الـbackends المميّزة المطلوبة (manager + الروسترات الثلاثة) ويعمل `Env::which` عليها. أي CLI ناقص → يقف فوراً برسالة واضحة: `agent CLI not found on PATH: codex — install it, or change the [agent] models`. مفيش lock بيتكتب، مفيش turn بيتحرق.

**(2) دفاع في العمق:** كانت ثغرة — لو الـbinary اختفى وسط الرَن، الـspawn بيفشل بـio-error وكان `Worker::fault` يصنّفه **Transient** → retry عبثي. دلوقتي `no such file`/`executable file not found`/`command not found` → **Fatal** → يقف فوراً. (مغطّى بـtest case جديد.)

**القرار التقني:** الـpreflight = فحص **وجود** (`which`) — رخيص وفوري ويمسك السبب #1 (مش متثبّت). الـ**"شغّال فعلاً"** (auth/quota منتهية) بيتمسك في أول turn تدريب بالـfault handling (`Fatal`/`Exhausted` → وقفة واضحة resumable) — عمل turn فعلي لكل backend في الـpreflight = حرق tokens بلا داعي طالما أول turn بيكشفها.

gate أخضر (clippy + test).

تاكد من ان السيناريو التالى شغال بمثالية : اولا الاداة قبل تهئية الموديلات بتقوم بفحص الاوامر والباينريز مثل claude, codex او اى اوامر اخرى بتستدعيها فى العمليات , لازم كلها تكون مثبته فى النظام وشغالة تماما والا الاداة تطلع خطأ صريح تقول فى اعتماديات كذا وكذا مش موجودة او مش شغالة وده يحصل قبل مايتم عمل start لاى حاجة وده نعمله اصلا امر اسمه agentx doctor يتاكد من فحص كل حاجة ويطبع الاخطاء واذا نجح وكل شئ صحيح يقوم بارجاع true او false اذا هناك اعتماديات مفقودة , وبكدا اوامر مثل start or restart هاتستدعيه بعد امر init مباشرة , بعد كدا الاداة لما تتاكد من كل شئ تقوم بقا بمرحلة تهئية المدير ثم المعمارين ثم المنفذين ثم المختبرين , ثم بعد ذلك مرحلة التاكيد تمر عليهم كلهم تانى تاكد على كل واحد دوره وهل فهم كل شئ ولا لا ويفهم تانى , ثم بعد ذلك مرحلة المدير البرومبت اللى يخليه ينشا ال requires او ياكد عليها وطبعا المرحلة دى مهمة المدير لازم يراعى هل الاداة هاتشتغل من جديد ولا هاتعمل resume على ال state قديم يعنى لازم الاداة تمررله ال state علشان يعرف احنا واقفين فين وماينشاش ال requires اللى تم انشاءها ويعرف الاداة فى انهى مرحلة ويقرا كل الاعدادات الحالية بما فيهم ملفات ال reporst, requires, tasks, tests, rounds والمراجعات بتاعته طبعا , لازم المدير يكون دراعى اليمين جوة الاداة يعنى فاهم كل شئ كويس اوى , وهو ده دور البرومبت بتاع المدير اللى بعد التهئية والتاكيد , يعنى المدير له تهئية وتاكيد زى باقى الموديلات ولكن بعد ده كله قبل البدا له برومبت اضافى زى ماقولنا انه يعمل ال requires واللى هو طبعا فيه بيعرف دوره اكتر ويشوف كل ال cache dir كويس ويدرسه علشان يعرف اذا هايعمل requires ولا الاداة هاتكمل , ثم بعد ذلك مرحلة المعمارين بجوالتهم وجولة المدير معاهم وكذلك المنفذين والمختبرين ولازم طبعا زى ماقولنا جولة المدير مع اى دور لازم تعمل reset لل round counter بحيث سابقا انا لاحظت ان بعد مراجعة المدير وقال لازم نرجع نعدل شئ لاحظة ان عداد ال round كان 2/5 والمفروض انه يكون 1/5 علشان دى اول دورة فى جولة المدير , وكمان المختبرين زى ماقولنا بيعملو اختبارات حقيقية فى المشروع ولا لا على حسب ال tests field سواء كان فى ملف manifest او فى فلاج ولكن طبعا مش هاتفرق لان اى فلاجات لازم الاول تتحول ل manifest spec fields فى الملف او تتخزن فى ال consts , وبكدا مفيش اى داعى لل #[test] داخل الاداة ياصاحبى , انطلق وظبط كل القرارات دى باحسن شكل ممكن ارجوك

مهمة كبيرة، فكّكتها لـ٥ بنود. أبدأ بالتنفيذ منظّم.

**(1) أشيل الـ`#[cfg(test)]` من الأداة** (المستخدم صريح):

**(2) `agentx doctor`** — تعريف الأمر:

Now replace `ensure_agents` with the doctor machinery (shared by `doctor` + `start`):

أضيف `PROBE_TIMEOUT` const (بدل CONSULT الطويل) + الـimport:

**(3) Round counter reset** — أقرأ run_phase + phase_tasks الحاليين:

**(4) MANAGER_INTAKE resume-aware** — المدير يعرف fresh/resume والملفات الموجودة فمايعيدش إنشاء requires. أعدّل Compose + الـprompt + الاستدعاء:

Now add `{state}` to the MANAGER_INTAKE prompt:

Gate after all 4 waves:

clippy أخضر. أجرّب `doctor` فعلياً لإثبات السلوك:

`doctor` شغّال — كشف claude 2.1.178. أختبر مسار الفشل + فحص sh مع gate:

`doctor` شغّال بمثالية: ✓ versions، ✗ بـexit 1 عند الفقد، يفحص sh مع الـgate. أوثّق + release gate. README:

Release gate + verify no test in tree:

الـgate كامل أخضر: صفر tests ✓، release **1.82M**. أحدّث الذاكرة (الـgate اتغيّر — مفيش test بقرار المستخدم + الميزات الجديدة):

نفّذت كل البنود ✅ — السيناريو كامل شغّال بمثالية:

| البند | الحالة |
|---|---|
| **`agentx doctor`** | يجمّع الـbackends المطلوبة (claude/codex من الروستر) + `sh` للـgate، يفحص كل واحد (`which` + `--version`)، يطبع `✓ claude 2.1.178` / `✗ not found`، يرجّع exit 0/1. **start/restart بينادوه قبل أي تهيئة** → يقف برسالة صريحة لو حاجة ناقصة. مُختبَر حيّ (claude✓ codex✓ sh✓ · empty-PATH→✗ exit 1) |
| **التهيئة→التأكيد→intake** | الترتيب قائم (manager→architects→executors→testers، ثم lap تأكيد، ثم intake) |
| **المدير resume-aware** | الـintake بقى يمرّر له `{state}`: FRESH ولا RESUME + عدد ملفات requires/tasks الموجودة + أمر صريح "اقرأ الموجود، كمّل الترقيم، **لا تعيد إنشاء** ما بُني منه tasks". يغطّي الـcrash وسط الـintake |
| **round counter reset** | الـbug الحقيقي اللي لاحظته (2/5 بدل 1/5): `current_round` كان بيتسرّب عبر حدود الـtask. اتصلّح في `run_phase` + `phase_tasks` → كل جولة مدير تبدأ من 1 نظيف |
| **flags→spec/consts** | قائم (كل الفلاجات بتتكتب في Agentx.toml أو consts) |
| **مفيش `#[test]`** | اتشال — الشجرة test-free، الـgate بقى clippy + build بس |

gate أخضر بالكامل · release **1.82M** · الذاكرة متحدّثة (الـdoctor + round fix + intake + قرار "لا tests").

عاوز اعمل امر لمراقبة الحالة يعنى يكون مثل امر status ولكنه realtime , ممكن نسميه مثلا agentx tail ييستدعى ال status كل ثانية او زى ماتعملها المهم يكون امر نظيف وامن واداء سريع مش بياثر على الاداء , اعمله بطريق شيك تماما , ممكن بدل ماتعمل امر كامل تخليه فلاج مثل agentx status --tail زى ماتحب شوف ايه الافضل ونفذه باقوى شكل ممكن باداء خارق ومعلومات صادقة

Already a `tail` exists. Let me read it.

موجود فعلاً ومتطبّق. أتأكد إنه متوصّل صح + الـgate.

متطبّق بالكامل بالفعل: `agentx status --tail` (`-t`). نظيف — fallback one-shot لو مش TTY، يحجب الإشارات، يخفي/يرجّع الكيرسر، `Ui::home()` = home+clear-to-end (مفيش وميض)، تحديث ~1s بـ10×100ms abort-aware، يخرج تلقائي على Completed/Failed/Ctrl+C، قراءة state.json رخيصة مش بتأثر على الرَن. أتأكد الـgate أخضر.

الأمر موجود ومتطبّق بالفعل: **`agentx status --tail`** (`-t`). نظيف، آمن، أداء خفيف، gate أخضر. مفيش حاجة محتاجة تتعمل.

تاكد ان كل الاوامر ماهى الا رابرز فوق App layer وحتى الفلاجات تكون معلمات او دوال برده ل App layer , اياك يكون اى بزنس لوجيك او دوال للاداة داخل ui or cli , لازم الاداة تشتغل صح تماما لو تم التعامل معها كمكتبة داخل مشروع رست اخر حتى ال --tail ده برده معلم او دالة داخل App , انطلق وتاكد ان طبقة cli, ui رابر نظيف تماما بيمرر الفلاجات كمعلمات او بيستدعى دوال معينة داخل ال App على حسب الامر او الفلاج , انطلق

cli.rs نظيف — كله dispatch لـ`App::*` والفلاجات بتتمرّر كـ`Flags`. أتأكد من ui.rs (لازم presentation بحت) وإن كل دوال App عامة (lib parity).

نظيف تماماً — مفيش حاجة محتاجة تتعدّل:

- **cli.rs**: dispatch بحت، كل أمر → `App::*`، الفلاجات بتتمرّر كـ`Flags` params (الوحيد غير ده `Self::help` = طباعة clap help، CLI أصيل مش business logic).
- **ui.rs**: صفر مراجع لـJourney/Spec/Config/Paths/Proc/File/Dir/Project/Train — presentation بحت (ANSI/owo-colors).
- **الـ13 أمر كلهم `pub fn` على App** → قابلين للاستدعاء كمكتبة. `--tail` = مجرد `bool` param لـ`App::status(dir, tail)`؛ والـtail internals (`watch`/`status_once`/`active_key`/`pid_line`) كلها جوّه App. الـlib≡CLI parity قائم.

تمام الغى فلاجات ال model, effort ومفيش مشكلة الفلاجات العامة تتقبل ف stop, clean مثل --gate, --inspire مايضرش , كمان هانعمل امر sync يكون مثل reset ولكن ده مش بيدوس على ال history , ده فقط بيدوس على كل ال train/xxx/{contracts, overview, skills, requires, about.md} يعنى الافضل تخليه يدوس على الكل عدا ال history بتاع كل المشاريع اللى فى مركز التدريب وبكدا يبقا اسمه على مسمى sync بيزامن التدريب اللى فى الاداة ويسيب التدريب السابق بتاع history بتاع المشاريع زى ماهو , ونسيب reset زى ماهو كدا يبقا برده اسم على مسمى اعد حالة المشروع , كمان امر clean غيره خلى اسمه clear افضل وطبعا ظبط كل المؤشرات بالاوامر الجديدة دى فى الاداة وملف README.md وملف CHANGELOG.md , وبكدا ال model, effort, agent كل ده اصبح فى ملف ال Agentx.toml مفيش داعى للفلاجات بقا خلاص , المهم اتاكد ان فلاج -t, --tests موجود لانك ماذكرتوش ده لازم يكون فلاج لاوامر ال init|start|restart زى فلاجات ال --gate, --inspire بالظبط لانه بياثر على قيمة ال tests زى امر ال inspire اللى بياثر على project.inspire field , انطلق

فهمت كل المطلوب. أقرأ الملفات المتأثرة الأول.

أعمل recon سريع لكل مراجع `clean`/`--model`/`--effort` قبل التعديل.

كله واضح. أنفّذ. **arch.rs:**

**cli.rs:**

**index.rs** — spawn_background flag forwarding:

Auto-clean message + `restart`/`clear` rename:

`prepare` + `apply_flags` (add tests, drop engine flags):

Add `App::sync` after `reset`:

**spec.rs** — `parse_tests`:

**project.rs** — rename `clean`→`clear`:

**train.rs** — `sync` (overwrite all but `history/`):

Gate:

أخضر. أتأكد من الـCLI فعلياً:

Functional test (no AI — preset inspire+gate):

flag/clear/sync سليم. مش هـشغّل `sync` فعليًا عشان مايدوسش على `~/.agentx` الحقيقي بتاعك (متحقَّق بالـbuild + المنطق). دلوقتي README + CHANGELOG.

**CHANGELOG:**

Final gate:

نفّذت. ✅

- شِلت فلاجات `--model`/`--effort` (model/effort بقت في `Agentx.toml` `[claude]`/`[codex]` بس).
- `-t/--tests <BOOL>` global على init/start/restart (يكتب `[project].tests`؛ يرفض قيمة غلط). نقلت `--tail` لـ`-f` لتفادي تصادم `-t`.
- أمر `sync` جديد: يزامن `overview/contracts/skills/requires/about` ويسيب `history/` لكل المشاريع. `reset` زي ماهو (wipe كامل).
- `clean` → `clear` (الأمر + الرسائل + `Project::clear`).
- README + CHANGELOG متسقين. gate أخضر، ومتحقَّق فعليًا (tests flag valid/invalid، clear، الـhelp). مشغلتش `sync` الحقيقي عشان مايدوسش على `~/.agentx` بتاعك.

