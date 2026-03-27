import json
import sys

with open("root_report.json") as f:
    data = json.load(f)

for spec in data.get("specifications", []):
    if "message-body" in spec.get("target", ""):
        print("TARGET:", spec.get("target", ""))
        for sec in spec.get("sections", []):
            sid = sec.get("id", "")
            for req in sec.get("requirements", []):
                st = req.get("status", {})
                hi = st.get("implementation", False)
                ht = st.get("test", False)
                if not hi or not ht:
                    m = []
                    if not hi:
                        m.append("impl")
                    if not ht:
                        m.append("test")
                    c = req.get("content", "")[:150]
                    lv = req.get("level", "")
                    print(f"GAP: {sid} | {lv} | {','.join(m)} | {c}")
        print("---ALL SECTIONS---")
        for sec in spec.get("sections", []):
            sid = sec.get("id", "")
            total = len(sec.get("requirements", []))
            covered = sum(1 for r in sec.get("requirements", []) if r.get("status", {}).get("implementation", False) and r.get("status", {}).get("test", False))
            print(f"  {sid}: {covered}/{total}")
